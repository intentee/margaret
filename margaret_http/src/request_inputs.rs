use std::collections::HashMap;
use std::net::SocketAddr;

use cookie::Cookie;
use http::HeaderMap;
use http::Method;
use http::Uri;
use http::header::COOKIE;

use crate::body_class::BodyClass;
use crate::collect_limited::collect_limited;
use crate::form_field::FormField;
use crate::form_fields::form_fields;
use crate::multipart_body::MultipartBody;
use crate::request_body::RequestBody;
use crate::request_error::RequestError;
use crate::server_params::ServerParams;
use crate::upload_config::UploadConfig;
use crate::uploaded_file::UploadedFile;

fn unspecified_addr() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], 0))
}

fn parse_cookies(headers: &HeaderMap) -> Result<HashMap<String, String>, RequestError> {
    let mut cookies = HashMap::new();
    let Some(raw) = headers.get(COOKIE) else {
        return Ok(cookies);
    };

    for parsed in Cookie::split_parse(raw.to_str()?) {
        let cookie = parsed?;
        let name = cookie.name().to_string();
        let value = cookie.value().to_string();

        cookies.entry(name).or_insert(value);
    }

    Ok(cookies)
}

fn index_fields(fields: Vec<FormField>) -> HashMap<String, String> {
    let mut indexed = HashMap::new();

    for FormField { name, value } in fields {
        indexed.entry(name).or_insert(value);
    }

    indexed
}

fn index_files(files: Vec<UploadedFile>) -> HashMap<String, UploadedFile> {
    let mut indexed = HashMap::new();

    for file in files {
        let field_name = file.field_name().to_string();

        indexed.entry(field_name).or_insert(file);
    }

    indexed
}

pub struct RequestInputs {
    pub cookies: HashMap<String, String>,
    pub files: HashMap<String, UploadedFile>,
    pub json: Option<serde_json::Value>,
    pub form: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub server: ServerParams,
}

impl RequestInputs {
    pub(crate) fn empty(method: Method, path: String) -> Self {
        Self {
            cookies: HashMap::new(),
            files: HashMap::new(),
            json: None,
            form: HashMap::new(),
            query: HashMap::new(),
            server: ServerParams::new(
                method,
                path,
                String::new(),
                unspecified_addr(),
                HeaderMap::new(),
            ),
        }
    }

    pub(crate) async fn parse(
        method: Method,
        uri: &Uri,
        headers: HeaderMap,
        remote_addr: SocketAddr,
        body: RequestBody,
        upload_config: &UploadConfig,
    ) -> Result<Self, RequestError> {
        let cookies = parse_cookies(&headers)?;
        let query = index_fields(form_fields(uri.query().unwrap_or_default().as_bytes()));
        let mut files = HashMap::new();
        let mut json: Option<serde_json::Value> = None;
        let mut form = HashMap::new();

        match BodyClass::from_headers(&headers)? {
            BodyClass::Multipart { boundary } => {
                let MultipartBody {
                    files: parsed_files,
                    post: parsed_post,
                } = MultipartBody::parse(body, boundary, upload_config).await?;

                files = index_files(parsed_files);
                form = index_fields(parsed_post);
            }
            BodyClass::UrlEncoded => {
                form = index_fields(form_fields(
                    &collect_limited(body, upload_config.max_body_size()).await?,
                ));
            }
            BodyClass::Json => {
                let bytes = collect_limited(body, upload_config.max_body_size()).await?;

                json = Some(
                    serde_json::from_slice(&bytes)
                        .map_err(|source| RequestError::MalformedJson { source })?,
                );
            }
            BodyClass::Other => {}
        }

        let server = ServerParams::new(
            method,
            uri.path().to_string(),
            uri.query().unwrap_or_default().to_string(),
            remote_addr,
            headers,
        );

        Ok(Self {
            cookies,
            files,
            json,
            form,
            query,
            server,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use bytes::Bytes;
    use http::HeaderMap;
    use http::HeaderValue;
    use http::Method;
    use http::Uri;
    use http::header::CONTENT_TYPE;
    use http::header::COOKIE;
    use http_body_util::BodyExt;
    use http_body_util::Full;
    use tempfile::TempDir;
    use tempfile::tempdir;

    use super::RequestInputs;
    use super::unspecified_addr;
    use crate::request_body::RequestBody;
    use crate::request_error::RequestError;
    use crate::upload_config::UploadConfig;

    fn boxed(bytes: &'static [u8]) -> RequestBody {
        Full::new(Bytes::from_static(bytes))
            .map_err(|error: Infallible| match error {})
            .boxed_unsync()
    }

    const MULTIPART: &[u8] = b"--X\r\nContent-Disposition: form-data; name=\"title\"\r\n\r\nhello\r\n--X\r\nContent-Disposition: form-data; name=\"avatar\"; filename=\"face.png\"\r\nContent-Type: image/png\r\n\r\nPNG\r\n--X\r\nContent-Disposition: form-data; name=\"raw\"; filename=\"raw.bin\"\r\n\r\nDATA\r\n--X--\r\n";

    fn upload_in(directory: &TempDir) -> UploadConfig {
        UploadConfig::enabled(directory.path().to_path_buf())
    }

    async fn parse_inputs(
        content_type: Option<&str>,
        target: &str,
        body: &'static [u8],
        upload_config: &UploadConfig,
    ) -> Result<RequestInputs, RequestError> {
        let mut headers = HeaderMap::new();

        if let Some(value) = content_type {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_str(value).expect("a header value"),
            );
        }

        let uri: Uri = target.parse().expect("a valid uri");

        RequestInputs::parse(
            Method::POST,
            &uri,
            headers,
            unspecified_addr(),
            boxed(body),
            upload_config,
        )
        .await
    }

    async fn parse_with_cookie(value: &[u8]) -> Result<RequestInputs, RequestError> {
        let mut headers = HeaderMap::new();

        headers.insert(
            COOKIE,
            HeaderValue::from_bytes(value).expect("a header value"),
        );

        let uri: Uri = "/".parse().expect("a valid uri");

        RequestInputs::parse(
            Method::GET,
            &uri,
            headers,
            unspecified_addr(),
            boxed(b""),
            &UploadConfig::Disabled,
        )
        .await
    }

    #[tokio::test]
    async fn parses_cookies_from_the_header() {
        let inputs = parse_with_cookie(b"session=abc; theme=dark")
            .await
            .expect("the request parses");

        assert_eq!(
            inputs.cookies.get("session").map(String::as_str),
            Some("abc")
        );
        assert_eq!(
            inputs.cookies.get("theme").map(String::as_str),
            Some("dark")
        );
    }

    #[tokio::test]
    async fn has_no_cookies_without_a_cookie_header() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parse_inputs(None, "/", b"", &upload_in(&directory))
            .await
            .expect("the request parses");

        assert!(inputs.cookies.is_empty());
    }

    #[tokio::test]
    async fn rejects_a_malformed_cookie_header() {
        let message = parse_with_cookie(b"=nameless")
            .await
            .err()
            .unwrap()
            .to_string();

        assert!(message.contains("could not be parsed"));
    }

    #[tokio::test]
    async fn rejects_a_non_ascii_cookie_header() {
        assert!(parse_with_cookie(&[0xC0, 0xC1]).await.is_err());
    }

    #[tokio::test]
    async fn parses_query_string_variables() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parse_inputs(
            None,
            "/search?term=rust&page=2",
            b"",
            &upload_in(&directory),
        )
        .await
        .expect("the request parses");

        assert_eq!(inputs.query.get("term").map(String::as_str), Some("rust"));
        assert_eq!(inputs.server.query_string(), "term=rust&page=2");
        assert!(inputs.json.is_none());
    }

    #[tokio::test]
    async fn parses_urlencoded_post_variables() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parse_inputs(
            Some("application/x-www-form-urlencoded"),
            "/login",
            b"username=margaret&password=secret",
            &upload_in(&directory),
        )
        .await
        .expect("the request parses");

        assert_eq!(
            inputs.form.get("username").map(String::as_str),
            Some("margaret")
        );
    }

    #[tokio::test]
    async fn parses_a_json_body() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parse_inputs(
            Some("application/json; charset=utf-8"),
            "/articles",
            b"{\"title\":\"hello\"}",
            &upload_in(&directory),
        )
        .await
        .expect("the request parses");

        assert!(inputs.json == Some(serde_json::json!({ "title": "hello" })));
    }

    #[tokio::test]
    async fn accepts_a_json_body_while_uploads_are_disabled() {
        let inputs = parse_inputs(
            Some("application/json"),
            "/articles",
            b"{\"title\":\"hello\"}",
            &UploadConfig::Disabled,
        )
        .await
        .expect("a json body parses without uploads enabled");

        assert!(inputs.json == Some(serde_json::json!({ "title": "hello" })));
    }

    #[tokio::test]
    async fn distinguishes_a_json_null_body_from_an_absent_body() {
        let directory = tempdir().expect("a temporary directory");
        let provided = parse_inputs(
            Some("application/json"),
            "/",
            b"null",
            &upload_in(&directory),
        )
        .await
        .expect("the request parses");

        assert!(provided.json == Some(serde_json::Value::Null));

        let absent = parse_inputs(None, "/", b"", &upload_in(&directory))
            .await
            .expect("the request parses");

        assert!(absent.json.is_none());
    }

    #[tokio::test]
    async fn rejects_a_malformed_json_body() {
        let directory = tempdir().expect("a temporary directory");
        let message = parse_inputs(
            Some("application/json"),
            "/",
            b"{not json",
            &upload_in(&directory),
        )
        .await
        .err()
        .unwrap()
        .to_string();

        assert!(message.contains("not valid JSON"));
    }

    #[tokio::test]
    async fn parses_multipart_fields_and_streams_files_to_disk() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parse_inputs(
            Some("multipart/form-data; boundary=X"),
            "/upload",
            MULTIPART,
            &upload_in(&directory),
        )
        .await
        .expect("the request parses");

        assert_eq!(inputs.form.get("title").map(String::as_str), Some("hello"));
        assert_eq!(inputs.files.len(), 2);

        let avatar = inputs
            .files
            .get("avatar")
            .expect("the avatar file is present");
        assert_eq!(avatar.file_name(), "face.png");
        assert_eq!(avatar.content_type(), "image/png");
        assert_eq!(avatar.size(), 3);
        assert_eq!(
            std::fs::read(avatar.path()).expect("the temp file is readable"),
            b"PNG"
        );

        let raw = inputs.files.get("raw").expect("the raw file is present");
        assert_eq!(raw.content_type(), "application/octet-stream");
        assert_eq!(
            std::fs::read(raw.path()).expect("the temp file is readable"),
            b"DATA"
        );
    }

    #[tokio::test]
    async fn rejects_an_uploaded_file_when_uploads_are_disabled() {
        let message = parse_inputs(
            Some("multipart/form-data; boundary=X"),
            "/upload",
            MULTIPART,
            &UploadConfig::Disabled,
        )
        .await
        .err()
        .unwrap()
        .to_string();

        assert!(message.contains("file uploads are disabled"));
    }

    #[tokio::test]
    async fn rejects_multipart_without_a_boundary() {
        let directory = tempdir().expect("a temporary directory");
        let message = parse_inputs(
            Some("multipart/form-data"),
            "/upload",
            MULTIPART,
            &upload_in(&directory),
        )
        .await
        .err()
        .unwrap()
        .to_string();

        assert!(message.contains("missing its Content-Type boundary"));
    }

    #[tokio::test]
    async fn rejects_an_oversized_multipart_stream() {
        let directory = tempdir().expect("a temporary directory");
        let upload_config = UploadConfig::Enabled {
            directory: directory.path().to_path_buf(),
            max_size: 8,
        };
        let message = parse_inputs(
            Some("multipart/form-data; boundary=X"),
            "/upload",
            MULTIPART,
            &upload_config,
        )
        .await
        .err()
        .unwrap()
        .to_string();

        assert!(message.contains("upload limit"));
    }

    #[tokio::test]
    async fn rejects_an_oversized_buffered_body() {
        let directory = tempdir().expect("a temporary directory");
        let upload_config = UploadConfig::Enabled {
            directory: directory.path().to_path_buf(),
            max_size: 8,
        };
        let message = parse_inputs(
            Some("application/x-www-form-urlencoded"),
            "/login",
            b"field=0123456789",
            &upload_config,
        )
        .await
        .err()
        .unwrap()
        .to_string();

        assert!(message.contains("8 byte upload limit"));
    }

    #[tokio::test]
    async fn ignores_a_body_with_an_unrecognized_content_type() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parse_inputs(Some("text/plain"), "/", b"whatever", &upload_in(&directory))
            .await
            .expect("the request parses");

        assert!(inputs.form.is_empty());
        assert!(inputs.files.is_empty());
        assert!(inputs.json.is_none());
    }

    #[tokio::test]
    async fn maps_a_temporary_file_failure_to_upload_temp_file() {
        let upload_config = UploadConfig::enabled("/margaret-nonexistent-upload-directory".into());
        let message = parse_inputs(
            Some("multipart/form-data; boundary=X"),
            "/upload",
            MULTIPART,
            &upload_config,
        )
        .await
        .err()
        .unwrap()
        .to_string();

        assert!(message.contains("temporary file could not be created"));
    }

    #[tokio::test]
    async fn reports_a_malformed_multipart_body() {
        let directory = tempdir().expect("a temporary directory");
        let body: &[u8] = b"--X\r\nContent-Disposition: form-data; name=\"f\"\r\n\r\nunterminated";
        let message = parse_inputs(
            Some("multipart/form-data; boundary=X"),
            "/upload",
            body,
            &upload_in(&directory),
        )
        .await
        .err()
        .unwrap()
        .to_string();

        assert!(message.contains("multipart request body could not be parsed"));
    }

    #[tokio::test]
    async fn reports_a_truncated_uploaded_file() {
        let directory = tempdir().expect("a temporary directory");
        let body: &[u8] =
            b"--X\r\nContent-Disposition: form-data; name=\"f\"; filename=\"a\"\r\n\r\nunterminated";
        let message = parse_inputs(
            Some("multipart/form-data; boundary=X"),
            "/upload",
            body,
            &upload_in(&directory),
        )
        .await
        .err()
        .unwrap()
        .to_string();

        assert!(message.contains("multipart request body could not be parsed"));
    }

    #[tokio::test]
    async fn deletes_uploaded_temporary_files_when_the_inputs_drop() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parse_inputs(
            Some("multipart/form-data; boundary=X"),
            "/upload",
            MULTIPART,
            &upload_in(&directory),
        )
        .await
        .expect("the request parses");
        let path = inputs
            .files
            .values()
            .next()
            .expect("an uploaded file is present")
            .path()
            .to_path_buf();

        assert!(path.exists());

        drop(inputs);

        assert!(!path.exists());
    }
}
