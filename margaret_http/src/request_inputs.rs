use std::collections::HashMap;

use cookie::Cookie;
use http::Method;
use http::header::COOKIE;

use crate::body_class::BodyClass;
use crate::body_limit::BodyLimit;
use crate::collect_limited::collect_limited;
use crate::form_field::FormField;
use crate::form_fields::form_fields;
use crate::multipart_body::MultipartBody;
use crate::request_body::RequestBody;
use crate::request_error::RequestError;
use crate::request_outcome::RequestOutcome;
use crate::request_rejection::RequestRejection;
use crate::server_params::ServerParams;
use crate::upload_config::UploadConfig;
use crate::uploaded_file::UploadedFile;

fn parse_cookies(server: &ServerParams) -> RequestOutcome<HashMap<String, String>> {
    let mut cookies = HashMap::new();
    let Some(raw) = server.header(&COOKIE) else {
        return RequestOutcome::Parsed(cookies);
    };

    for parsed in Cookie::split_parse(raw) {
        let cookie = match parsed {
            Ok(cookie) => cookie,
            Err(source) => {
                return RequestOutcome::Rejected(RequestRejection::MalformedCookie { source });
            }
        };
        let name = cookie.name().to_string();

        if cookies.contains_key(&name) {
            return RequestOutcome::Rejected(RequestRejection::DuplicateCookie { name });
        }

        cookies.insert(name, cookie.value().to_string());
    }

    RequestOutcome::Parsed(cookies)
}

fn index_fields(
    fields: Vec<FormField>,
    reject_duplicate: fn(String) -> RequestRejection,
) -> RequestOutcome<HashMap<String, String>> {
    let mut indexed = HashMap::with_capacity(fields.len());

    for FormField { name, value } in fields {
        if indexed.contains_key(&name) {
            return RequestOutcome::Rejected(reject_duplicate(name));
        }

        indexed.insert(name, value);
    }

    RequestOutcome::Parsed(indexed)
}

fn index_files(files: Vec<UploadedFile>) -> RequestOutcome<HashMap<String, UploadedFile>> {
    let mut indexed = HashMap::with_capacity(files.len());

    for file in files {
        let field_name = file.field_name().to_string();

        if indexed.contains_key(&field_name) {
            return RequestOutcome::Rejected(RequestRejection::DuplicateUploadField {
                name: field_name,
            });
        }

        indexed.insert(field_name, file);
    }

    RequestOutcome::Parsed(indexed)
}

fn parse_query(server: &ServerParams) -> RequestOutcome<HashMap<String, String>> {
    index_fields(form_fields(server.query_string().as_bytes()), |name| {
        RequestRejection::DuplicateQueryParameter { name }
    })
}

struct ParsedBody {
    files: HashMap<String, UploadedFile>,
    form: HashMap<String, String>,
    json: Option<serde_json::Value>,
}

impl ParsedBody {
    fn empty() -> Self {
        Self {
            files: HashMap::new(),
            form: HashMap::new(),
            json: None,
        }
    }
}

async fn parse_body(
    server: &ServerParams,
    body: RequestBody,
    body_limit: &BodyLimit,
    upload_config: &UploadConfig,
) -> Result<RequestOutcome<ParsedBody>, RequestError> {
    let body_class = match BodyClass::from_server_params(server) {
        RequestOutcome::Parsed(body_class) => body_class,
        RequestOutcome::Rejected(rejection) => return Ok(RequestOutcome::Rejected(rejection)),
    };

    match body_class {
        BodyClass::Multipart { boundary } => {
            let multipart = MultipartBody::parse(body, boundary, body_limit, upload_config).await?;
            let MultipartBody {
                files: parsed_files,
                post,
            } = match multipart {
                RequestOutcome::Parsed(multipart) => multipart,
                RequestOutcome::Rejected(rejection) => {
                    return Ok(RequestOutcome::Rejected(rejection));
                }
            };
            let files = match index_files(parsed_files) {
                RequestOutcome::Parsed(files) => files,
                RequestOutcome::Rejected(rejection) => {
                    return Ok(RequestOutcome::Rejected(rejection));
                }
            };
            let form =
                match index_fields(post, |name| RequestRejection::DuplicateFormField { name }) {
                    RequestOutcome::Parsed(form) => form,
                    RequestOutcome::Rejected(rejection) => {
                        return Ok(RequestOutcome::Rejected(rejection));
                    }
                };

            Ok(RequestOutcome::Parsed(ParsedBody {
                files,
                form,
                json: None,
            }))
        }
        BodyClass::UrlEncoded => {
            let bytes = match collect_limited(body, body_limit.max_bytes()).await {
                RequestOutcome::Parsed(bytes) => bytes,
                RequestOutcome::Rejected(rejection) => {
                    return Ok(RequestOutcome::Rejected(rejection));
                }
            };

            match index_fields(form_fields(&bytes), |name| {
                RequestRejection::DuplicateFormField { name }
            }) {
                RequestOutcome::Parsed(form) => Ok(RequestOutcome::Parsed(ParsedBody {
                    files: HashMap::new(),
                    form,
                    json: None,
                })),
                RequestOutcome::Rejected(rejection) => Ok(RequestOutcome::Rejected(rejection)),
            }
        }
        BodyClass::Json => {
            let bytes = match collect_limited(body, body_limit.max_bytes()).await {
                RequestOutcome::Parsed(bytes) => bytes,
                RequestOutcome::Rejected(rejection) => {
                    return Ok(RequestOutcome::Rejected(rejection));
                }
            };

            match serde_json::from_slice(&bytes) {
                Ok(json) => Ok(RequestOutcome::Parsed(ParsedBody {
                    files: HashMap::new(),
                    form: HashMap::new(),
                    json: Some(json),
                })),
                Err(source) => Ok(RequestOutcome::Rejected(RequestRejection::MalformedJson {
                    source,
                })),
            }
        }
        BodyClass::Other => Ok(RequestOutcome::Parsed(ParsedBody::empty())),
    }
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
    pub(crate) fn from_handshake(server: ServerParams) -> RequestOutcome<Self> {
        let cookies = match parse_cookies(&server) {
            RequestOutcome::Parsed(cookies) => cookies,
            RequestOutcome::Rejected(rejection) => return RequestOutcome::Rejected(rejection),
        };
        let query = match parse_query(&server) {
            RequestOutcome::Parsed(query) => query,
            RequestOutcome::Rejected(rejection) => return RequestOutcome::Rejected(rejection),
        };

        RequestOutcome::Parsed(Self {
            cookies,
            files: HashMap::new(),
            json: None,
            form: HashMap::new(),
            query,
            server,
        })
    }

    pub(crate) async fn parse(
        server: ServerParams,
        body: RequestBody,
        body_limit: &BodyLimit,
        upload_config: &UploadConfig,
    ) -> Result<RequestOutcome<Self>, RequestError> {
        let cookies = match parse_cookies(&server) {
            RequestOutcome::Parsed(cookies) => cookies,
            RequestOutcome::Rejected(rejection) => return Ok(RequestOutcome::Rejected(rejection)),
        };
        let query = match parse_query(&server) {
            RequestOutcome::Parsed(query) => query,
            RequestOutcome::Rejected(rejection) => return Ok(RequestOutcome::Rejected(rejection)),
        };
        let ParsedBody { files, form, json } = {
            let parsed_body = parse_body(&server, body, body_limit, upload_config).await?;

            match parsed_body {
                RequestOutcome::Parsed(parsed) => parsed,
                RequestOutcome::Rejected(rejection) => {
                    return Ok(RequestOutcome::Rejected(rejection));
                }
            }
        };

        Ok(RequestOutcome::Parsed(Self {
            cookies,
            files,
            json,
            form,
            query,
            server,
        }))
    }

    pub(crate) fn synthetic(method: Method, path: String) -> Self {
        Self {
            cookies: HashMap::new(),
            files: HashMap::new(),
            json: None,
            form: HashMap::new(),
            query: HashMap::new(),
            server: ServerParams::synthetic(method, path),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;
    use std::mem::discriminant;
    use std::net::SocketAddr;

    use bytes::Bytes;
    use http::HeaderValue;
    use http::Method;
    use http::Request;
    use http::header::CONTENT_TYPE;
    use http::header::COOKIE;
    use http::header::HOST;
    use http_body_util::BodyExt;
    use http_body_util::Full;
    use tempfile::TempDir;
    use tempfile::tempdir;

    use super::RequestInputs;
    use crate::body_limit::BodyLimit;
    use crate::request_body::RequestBody;
    use crate::request_error::RequestError;
    use crate::request_outcome::RequestOutcome;
    use crate::request_rejection::RequestRejection;
    use crate::server_params::ServerParams;
    use crate::singleton_request_header::SingletonRequestHeader;
    use crate::upload_config::UploadConfig;

    fn any_name() -> String {
        "name".to_string()
    }

    fn boxed(bytes: &'static [u8]) -> RequestBody {
        Full::new(Bytes::from_static(bytes))
            .map_err(|error: Infallible| match error {})
            .boxed_unsync()
    }

    const MULTIPART: &[u8] = b"--X\r\nContent-Disposition: form-data; name=\"title\"\r\n\r\nhello\r\n--X\r\nContent-Disposition: form-data; name=\"avatar\"; filename=\"face.png\"\r\nContent-Type: image/png\r\n\r\nPNG\r\n--X\r\nContent-Disposition: form-data; name=\"raw\"; filename=\"raw.bin\"\r\n\r\nDATA\r\n--X--\r\n";

    fn upload_in(directory: &TempDir) -> UploadConfig {
        UploadConfig::enabled(directory.path().to_path_buf())
    }

    fn head(
        method: Method,
        target: &str,
        headers: &[(&str, &[u8])],
    ) -> Result<ServerParams, RequestRejection> {
        let mut builder = Request::builder()
            .method(method)
            .uri(target)
            .header(HOST, "localhost");

        for (name, value) in headers {
            builder = builder.header(*name, HeaderValue::from_bytes(value).expect("a value"));
        }

        let parts = builder.body(()).expect("a request").into_parts().0;

        match ServerParams::from_parts(parts, SocketAddr::from(([127, 0, 0, 1], 0))) {
            RequestOutcome::Parsed(server) => Ok(server),
            RequestOutcome::Rejected(rejection) => Err(rejection),
        }
    }

    fn server_params(method: Method, target: &str, headers: &[(&str, &[u8])]) -> ServerParams {
        head(method, target, headers).expect("the request head is unambiguous")
    }

    #[test]
    fn rejects_a_repeated_cookie_header_before_the_cookies_are_read() {
        assert_eq!(
            discriminant(
                &head(
                    Method::GET,
                    "/",
                    &[(COOKIE.as_str(), b"a=1"), (COOKIE.as_str(), b"b=2")]
                )
                .err()
                .expect("a repeated Cookie header is ambiguous")
            ),
            discriminant(&RequestRejection::RepeatedSingletonHeader {
                header: SingletonRequestHeader::Cookie
            })
        );
    }

    async fn parse_inputs(
        content_type: Option<&str>,
        target: &str,
        body: &'static [u8],
        upload_config: &UploadConfig,
    ) -> Result<RequestOutcome<RequestInputs>, RequestError> {
        parse_inputs_with_limit(
            content_type,
            target,
            body,
            &BodyLimit::default(),
            upload_config,
        )
        .await
    }

    async fn parse_inputs_with_limit(
        content_type: Option<&str>,
        target: &str,
        body: &'static [u8],
        body_limit: &BodyLimit,
        upload_config: &UploadConfig,
    ) -> Result<RequestOutcome<RequestInputs>, RequestError> {
        let headers: Vec<(&str, &[u8])> = match content_type {
            Some(value) => vec![(CONTENT_TYPE.as_str(), value.as_bytes())],
            None => Vec::new(),
        };

        RequestInputs::parse(
            server_params(Method::POST, target, &headers),
            boxed(body),
            body_limit,
            upload_config,
        )
        .await
    }

    async fn outcome_with_limit(
        content_type: Option<&str>,
        target: &str,
        body: &'static [u8],
        body_limit: &BodyLimit,
        upload_config: &UploadConfig,
    ) -> Result<RequestInputs, RequestRejection> {
        match parse_inputs_with_limit(content_type, target, body, body_limit, upload_config)
            .await
            .expect("the request body is readable")
        {
            RequestOutcome::Parsed(inputs) => Ok(inputs),
            RequestOutcome::Rejected(rejection) => Err(rejection),
        }
    }

    async fn outcome(
        content_type: Option<&str>,
        target: &str,
        body: &'static [u8],
        upload_config: &UploadConfig,
    ) -> Result<RequestInputs, RequestRejection> {
        outcome_with_limit(
            content_type,
            target,
            body,
            &BodyLimit::default(),
            upload_config,
        )
        .await
    }

    async fn parsed(
        content_type: Option<&str>,
        target: &str,
        body: &'static [u8],
        upload_config: &UploadConfig,
    ) -> RequestInputs {
        outcome(content_type, target, body, upload_config)
            .await
            .expect("the request is unambiguous")
    }

    async fn rejected(
        content_type: Option<&str>,
        target: &str,
        body: &'static [u8],
        upload_config: &UploadConfig,
    ) -> RequestRejection {
        outcome(content_type, target, body, upload_config)
            .await
            .err()
            .expect("the request is ambiguous")
    }

    async fn rejected_with_limit(
        content_type: Option<&str>,
        target: &str,
        body: &'static [u8],
        body_limit: &BodyLimit,
        upload_config: &UploadConfig,
    ) -> RequestRejection {
        outcome_with_limit(content_type, target, body, body_limit, upload_config)
            .await
            .err()
            .expect("the request is ambiguous")
    }

    fn handshake_outcome(
        method: Method,
        target: &str,
        headers: &[(&str, &[u8])],
    ) -> Result<RequestInputs, RequestRejection> {
        match RequestInputs::from_handshake(server_params(method, target, headers)) {
            RequestOutcome::Parsed(inputs) => Ok(inputs),
            RequestOutcome::Rejected(rejection) => Err(rejection),
        }
    }

    async fn cookie_outcome(value: &[u8]) -> Result<RequestInputs, RequestRejection> {
        match RequestInputs::parse(
            server_params(Method::GET, "/", &[(COOKIE.as_str(), value)]),
            boxed(b""),
            &BodyLimit::default(),
            &UploadConfig::Disabled,
        )
        .await
        .expect("the request body is readable")
        {
            RequestOutcome::Parsed(inputs) => Ok(inputs),
            RequestOutcome::Rejected(rejection) => Err(rejection),
        }
    }

    #[test]
    fn builds_body_less_inputs_from_a_handshake() {
        let inputs = handshake_outcome(
            Method::GET,
            "/socket?room=lobby",
            &[(COOKIE.as_str(), b"session=abc")],
        )
        .expect("the handshake is unambiguous");

        assert_eq!(
            inputs.cookies.get("session").map(String::as_str),
            Some("abc")
        );
        assert_eq!(inputs.query.get("room").map(String::as_str), Some("lobby"));
        assert_eq!(inputs.server.path(), "/socket");
        assert_eq!(inputs.server.query_string(), "room=lobby");
        assert!(inputs.form.is_empty());
        assert!(inputs.files.is_empty());
        assert!(inputs.json.is_none());
    }

    #[test]
    fn rejects_a_handshake_with_a_malformed_cookie_header() {
        assert_eq!(
            discriminant(
                &handshake_outcome(Method::GET, "/socket", &[(COOKIE.as_str(), b"=nameless")])
                    .err()
                    .expect("a malformed cookie is rejected")
            ),
            discriminant(&RequestRejection::MalformedCookie {
                source: cookie::Cookie::parse("=nameless").expect_err("a malformed cookie")
            })
        );
    }

    #[test]
    fn rejects_a_handshake_with_a_duplicate_query_parameter() {
        assert_eq!(
            discriminant(
                &handshake_outcome(Method::GET, "/socket?id=1&id=2", &[])
                    .err()
                    .expect("a repeated query parameter is rejected")
            ),
            discriminant(&RequestRejection::DuplicateQueryParameter { name: any_name() })
        );
    }

    #[test]
    fn carries_a_synthetic_request_without_inputs() {
        let inputs = RequestInputs::synthetic(Method::GET, "/greeting".to_string());

        assert_eq!(inputs.server.path(), "/greeting");
        assert!(inputs.cookies.is_empty());
        assert!(inputs.query.is_empty());
    }

    #[tokio::test]
    async fn parses_cookies_from_the_header() {
        let inputs = cookie_outcome(b"session=abc; theme=dark")
            .await
            .expect("the cookie header is unambiguous");

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
    async fn rejects_a_repeated_cookie_name() {
        assert_eq!(
            discriminant(
                &cookie_outcome(b"session=abc; session=stolen")
                    .await
                    .err()
                    .expect("a repeated cookie name is rejected")
            ),
            discriminant(&RequestRejection::DuplicateCookie { name: any_name() })
        );
    }

    #[tokio::test]
    async fn has_no_cookies_without_a_cookie_header() {
        let directory = tempdir().expect("a temporary directory");

        assert!(
            parsed(None, "/", b"", &upload_in(&directory))
                .await
                .cookies
                .is_empty()
        );
    }

    #[tokio::test]
    async fn rejects_a_malformed_cookie_header() {
        assert_eq!(
            discriminant(
                &cookie_outcome(b"=nameless")
                    .await
                    .err()
                    .expect("a malformed cookie is rejected")
            ),
            discriminant(&RequestRejection::MalformedCookie {
                source: cookie::Cookie::parse("=nameless").expect_err("a malformed cookie")
            })
        );
    }

    #[tokio::test]
    async fn parses_query_string_variables() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parsed(
            None,
            "/search?term=rust&page=2",
            b"",
            &upload_in(&directory),
        )
        .await;

        assert_eq!(inputs.query.get("term").map(String::as_str), Some("rust"));
        assert_eq!(inputs.server.query_string(), "term=rust&page=2");
        assert!(inputs.json.is_none());
    }

    #[tokio::test]
    async fn rejects_a_repeated_query_parameter() {
        let directory = tempdir().expect("a temporary directory");

        assert_eq!(
            discriminant(&rejected(None, "/search?id=1&id=2", b"", &upload_in(&directory)).await),
            discriminant(&RequestRejection::DuplicateQueryParameter { name: any_name() })
        );
    }

    #[tokio::test]
    async fn parses_urlencoded_post_variables() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parsed(
            Some("application/x-www-form-urlencoded"),
            "/login",
            b"username=margaret&password=secret",
            &upload_in(&directory),
        )
        .await;

        assert_eq!(
            inputs.form.get("username").map(String::as_str),
            Some("margaret")
        );
    }

    #[tokio::test]
    async fn rejects_a_repeated_urlencoded_form_field() {
        let directory = tempdir().expect("a temporary directory");

        assert_eq!(
            discriminant(
                &rejected(
                    Some("application/x-www-form-urlencoded"),
                    "/login",
                    b"tag=x&tag=y",
                    &upload_in(&directory)
                )
                .await
            ),
            discriminant(&RequestRejection::DuplicateFormField { name: any_name() })
        );
    }

    #[tokio::test]
    async fn parses_a_json_body() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parsed(
            Some("application/json; charset=utf-8"),
            "/articles",
            b"{\"title\":\"hello\"}",
            &upload_in(&directory),
        )
        .await;

        assert!(inputs.json == Some(serde_json::json!({ "title": "hello" })));
    }

    #[tokio::test]
    async fn accepts_a_json_body_while_uploads_are_disabled() {
        let inputs = parsed(
            Some("application/json"),
            "/articles",
            b"{\"title\":\"hello\"}",
            &UploadConfig::Disabled,
        )
        .await;

        assert!(inputs.json == Some(serde_json::json!({ "title": "hello" })));
    }

    #[tokio::test]
    async fn distinguishes_a_json_null_body_from_an_absent_body() {
        let directory = tempdir().expect("a temporary directory");
        let provided = parsed(
            Some("application/json"),
            "/",
            b"null",
            &upload_in(&directory),
        )
        .await;

        assert!(provided.json == Some(serde_json::Value::Null));

        let absent = parsed(None, "/", b"", &upload_in(&directory)).await;

        assert!(absent.json.is_none());
    }

    #[tokio::test]
    async fn rejects_a_malformed_json_body() {
        let directory = tempdir().expect("a temporary directory");

        assert_eq!(
            discriminant(
                &rejected(
                    Some("application/json"),
                    "/",
                    b"{not json",
                    &upload_in(&directory)
                )
                .await
            ),
            discriminant(&RequestRejection::MalformedJson {
                source: serde_json::from_slice::<serde_json::Value>(b"{")
                    .expect_err("malformed json")
            })
        );
    }

    #[tokio::test]
    async fn parses_multipart_fields_and_streams_files_to_disk() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parsed(
            Some("multipart/form-data; boundary=X"),
            "/upload",
            MULTIPART,
            &upload_in(&directory),
        )
        .await;

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
    async fn rejects_a_repeated_multipart_upload_field() {
        let directory = tempdir().expect("a temporary directory");

        assert_eq!(discriminant(&rejected(
                Some("multipart/form-data; boundary=X"),
                "/upload",
                b"--X\r\nContent-Disposition: form-data; name=\"f\"; filename=\"a\"\r\n\r\nA\r\n--X\r\nContent-Disposition: form-data; name=\"f\"; filename=\"b\"\r\n\r\nB\r\n--X--\r\n",
                &upload_in(&directory)
            )
            .await), discriminant(&RequestRejection::DuplicateUploadField { name: any_name() }));
    }

    #[tokio::test]
    async fn rejects_a_repeated_multipart_form_field() {
        let directory = tempdir().expect("a temporary directory");

        assert_eq!(discriminant(&rejected(
                Some("multipart/form-data; boundary=X"),
                "/upload",
                b"--X\r\nContent-Disposition: form-data; name=\"t\"\r\n\r\nA\r\n--X\r\nContent-Disposition: form-data; name=\"t\"\r\n\r\nB\r\n--X--\r\n",
                &upload_in(&directory)
            )
            .await), discriminant(&RequestRejection::DuplicateFormField { name: any_name() }));
    }

    #[tokio::test]
    async fn rejects_an_uploaded_file_when_uploads_are_disabled() {
        assert_eq!(
            discriminant(
                &rejected(
                    Some("multipart/form-data; boundary=X"),
                    "/upload",
                    MULTIPART,
                    &UploadConfig::Disabled
                )
                .await
            ),
            discriminant(&RequestRejection::UploadsDisabled)
        );
    }

    #[tokio::test]
    async fn rejects_multipart_without_a_boundary() {
        let directory = tempdir().expect("a temporary directory");

        assert_eq!(
            discriminant(
                &rejected(
                    Some("multipart/form-data"),
                    "/upload",
                    MULTIPART,
                    &upload_in(&directory)
                )
                .await
            ),
            discriminant(&RequestRejection::MissingMultipartBoundary)
        );
    }

    #[tokio::test]
    async fn rejects_a_multipart_part_without_a_name() {
        let directory = tempdir().expect("a temporary directory");

        assert_eq!(
            discriminant(
                &rejected(
                    Some("multipart/form-data; boundary=X"),
                    "/upload",
                    b"--X\r\nContent-Disposition: form-data\r\n\r\nhello\r\n--X--\r\n",
                    &upload_in(&directory)
                )
                .await
            ),
            discriminant(&RequestRejection::NamelessMultipartField)
        );
    }

    #[tokio::test]
    async fn rejects_an_oversized_multipart_stream() {
        let directory = tempdir().expect("a temporary directory");

        assert_eq!(
            discriminant(
                &rejected_with_limit(
                    Some("multipart/form-data; boundary=X"),
                    "/upload",
                    MULTIPART,
                    &BodyLimit::new(8),
                    &upload_in(&directory),
                )
                .await
            ),
            discriminant(&RequestRejection::PayloadTooLarge { limit: 0 })
        );
    }

    #[tokio::test]
    async fn rejects_an_oversized_buffered_body() {
        assert_eq!(
            rejected_with_limit(
                Some("application/x-www-form-urlencoded"),
                "/login",
                b"field=0123456789",
                &BodyLimit::new(8),
                &UploadConfig::Disabled,
            )
            .await
            .to_string(),
            "the request body exceeds the 8 byte upload limit"
        );
    }

    #[tokio::test]
    async fn ignores_a_body_with_an_unrecognized_content_type() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parsed(Some("text/plain"), "/", b"whatever", &upload_in(&directory)).await;

        assert!(inputs.form.is_empty());
        assert!(inputs.files.is_empty());
        assert!(inputs.json.is_none());
    }

    #[tokio::test]
    async fn maps_a_temporary_file_failure_to_upload_temp_file() {
        let upload_config = UploadConfig::enabled("/margaret-nonexistent-upload-directory".into());
        let error = parse_inputs(
            Some("multipart/form-data; boundary=X"),
            "/upload",
            MULTIPART,
            &upload_config,
        )
        .await
        .err()
        .expect("an unusable upload directory is a system failure");

        assert_eq!(
            discriminant(&error),
            discriminant(&RequestError::UploadTempFile {
                source: std::io::Error::other("an unusable upload directory"),
            })
        );
    }

    #[tokio::test]
    async fn reports_a_malformed_multipart_body() {
        let directory = tempdir().expect("a temporary directory");

        assert_eq!(
            discriminant(
                &rejected(
                    Some("multipart/form-data; boundary=X"),
                    "/upload",
                    b"--X\r\nContent-Disposition: form-data; name=\"f\"\r\n\r\nunterminated",
                    &upload_in(&directory)
                )
                .await
            ),
            discriminant(&RequestRejection::MalformedMultipart {
                source: multer::Error::IncompleteStream
            })
        );
    }

    #[tokio::test]
    async fn reports_a_truncated_uploaded_file() {
        let directory = tempdir().expect("a temporary directory");

        assert_eq!(discriminant(&rejected(
                Some("multipart/form-data; boundary=X"),
                "/upload",
                b"--X\r\nContent-Disposition: form-data; name=\"f\"; filename=\"a\"\r\n\r\nunterminated",
                &upload_in(&directory)
            )
            .await), discriminant(&RequestRejection::MalformedMultipart { source: multer::Error::IncompleteStream }));
    }

    #[tokio::test]
    async fn deletes_uploaded_temporary_files_when_the_inputs_drop() {
        let directory = tempdir().expect("a temporary directory");
        let inputs = parsed(
            Some("multipart/form-data; boundary=X"),
            "/upload",
            MULTIPART,
            &upload_in(&directory),
        )
        .await;
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
