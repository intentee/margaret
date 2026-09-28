use std::net::SocketAddr;

use http::HeaderName;
use http::Method;
use http::Version;
use http::header::AUTHORIZATION;
use http::header::HOST;
use http::request::Parts;
use http::uri::Authority;

use crate::request_authorization::RequestAuthorization;
use crate::request_headers::RequestHeaders;
use crate::request_outcome::RequestOutcome;
use crate::request_path::RequestPath;
use crate::request_rejection::RequestRejection;

fn unspecified_addr() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], 0))
}

fn requires_host(version: Version) -> bool {
    version != Version::HTTP_09 && version != Version::HTTP_10
}

fn reconcile_authority(
    headers: &RequestHeaders,
    target_authority: Option<&Authority>,
    version: Version,
) -> RequestOutcome<()> {
    let host = match headers.get(&HOST) {
        Some(host) => match host.parse::<Authority>() {
            Ok(host) => Some(host),
            Err(source) => {
                return RequestOutcome::Rejected(RequestRejection::MalformedHost { source });
            }
        },
        None => None,
    };

    match (target_authority, host) {
        (Some(target), Some(host)) if *target != host => {
            RequestOutcome::Rejected(RequestRejection::AuthorityMismatch)
        }
        (None, None) if requires_host(version) => {
            RequestOutcome::Rejected(RequestRejection::MissingHost)
        }
        _ => RequestOutcome::Parsed(()),
    }
}

pub struct ServerParams {
    authorization: RequestAuthorization,
    headers: RequestHeaders,
    method: Method,
    path: RequestPath,
    query_string: String,
    remote_addr: SocketAddr,
}

impl ServerParams {
    pub(crate) fn from_parts(
        Parts {
            headers,
            method,
            uri,
            version,
            ..
        }: Parts,
        remote_addr: SocketAddr,
    ) -> RequestOutcome<Self> {
        let headers = match RequestHeaders::from_header_map(&headers) {
            RequestOutcome::Parsed(headers) => headers,
            RequestOutcome::Rejected(rejection) => return RequestOutcome::Rejected(rejection),
        };

        if let RequestOutcome::Rejected(rejection) =
            reconcile_authority(&headers, uri.authority(), version)
        {
            return RequestOutcome::Rejected(rejection);
        }

        let path = match RequestPath::from_request_target(uri.path()) {
            RequestOutcome::Parsed(path) => path,
            RequestOutcome::Rejected(rejection) => return RequestOutcome::Rejected(rejection),
        };

        RequestOutcome::Parsed(Self {
            authorization: RequestAuthorization::parse(headers.get(&AUTHORIZATION)),
            headers,
            method,
            path,
            query_string: uri.query().unwrap_or_default().to_string(),
            remote_addr,
        })
    }

    pub(crate) fn synthetic(method: Method, path: String) -> Self {
        Self {
            authorization: RequestAuthorization::Absent,
            headers: RequestHeaders::empty(),
            method,
            path: RequestPath::from_decoded(path),
            query_string: String::new(),
            remote_addr: unspecified_addr(),
        }
    }

    #[must_use]
    pub fn authorization(&self) -> &RequestAuthorization {
        &self.authorization
    }

    #[must_use]
    pub fn header(&self, name: &HeaderName) -> Option<&str> {
        self.headers.get(name)
    }

    #[must_use]
    pub fn method(&self) -> &str {
        self.method.as_str()
    }

    #[must_use]
    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    #[must_use]
    pub fn query_string(&self) -> &str {
        &self.query_string
    }

    #[must_use]
    pub fn remote_addr(&self) -> SocketAddr {
        self.remote_addr
    }
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use http::HeaderValue;
    use http::Method;
    use http::Request;
    use http::Version;
    use http::header::ACCEPT_ENCODING;
    use http::header::AUTHORIZATION;
    use http::header::HOST;
    use http::request::Parts;
    use http::uri::Authority;

    use super::ServerParams;
    use super::unspecified_addr;
    use crate::bearer_token::BearerToken;
    use crate::request_authorization::RequestAuthorization;
    use crate::request_outcome::RequestOutcome;
    use crate::request_rejection::RequestRejection;
    use crate::singleton_request_header::SingletonRequestHeader;

    fn parts_of(version: Version, target: &str, host: Option<&str>) -> Parts {
        let mut builder = Request::builder().method(Method::GET).uri(target);

        if let Some(host) = host {
            builder = builder.header(HOST, host);
        }

        builder
            .version(version)
            .body(())
            .expect("a request")
            .into_parts()
            .0
    }

    fn outcome_of(parts: Parts) -> Result<ServerParams, RequestRejection> {
        match ServerParams::from_parts(parts, unspecified_addr()) {
            RequestOutcome::Parsed(server) => Ok(server),
            RequestOutcome::Rejected(rejection) => Err(rejection),
        }
    }

    fn outcome(
        version: Version,
        target: &str,
        host: Option<&str>,
    ) -> Result<ServerParams, RequestRejection> {
        outcome_of(parts_of(version, target, host))
    }

    fn parse(version: Version, target: &str, host: Option<&str>) -> ServerParams {
        outcome(version, target, host).expect("the request head is unambiguous")
    }

    fn assert_rejects(
        version: Version,
        target: &str,
        host: Option<&str>,
        expected: &RequestRejection,
    ) {
        assert_eq!(
            discriminant(
                &outcome(version, target, host)
                    .err()
                    .expect("the request head is ambiguous")
            ),
            discriminant(expected)
        );
    }

    fn any_segment() -> String {
        "segment".to_string()
    }

    #[test]
    fn exposes_the_decoded_path_and_the_raw_query() {
        let server = parse(Version::HTTP_11, "/files/a%20b?page=2", Some("localhost"));

        assert_eq!(server.method(), "GET");
        assert_eq!(server.path(), "/files/a b");
        assert_eq!(server.query_string(), "page=2");
        assert_eq!(server.remote_addr(), unspecified_addr());
    }

    #[test]
    fn reports_an_empty_query_when_the_target_carries_none() {
        assert_eq!(
            parse(Version::HTTP_11, "/", Some("localhost")).query_string(),
            ""
        );
    }

    #[test]
    fn parses_the_authorization_once_with_the_request_head() {
        let mut parts = parts_of(Version::HTTP_11, "/", Some("localhost"));
        parts
            .headers
            .append(AUTHORIZATION, HeaderValue::from_static("Bearer abc"));

        let server = outcome_of(parts).expect("the request head is unambiguous");

        assert_eq!(
            server.authorization(),
            &RequestAuthorization::Bearer(BearerToken::new("abc".to_string()))
        );
    }

    #[test]
    fn reads_a_header_by_its_typed_name() {
        let mut parts = parts_of(Version::HTTP_11, "/", Some("localhost"));
        parts
            .headers
            .append(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));

        let server = outcome_of(parts).expect("the request head is unambiguous");

        assert_eq!(server.header(&ACCEPT_ENCODING), Some("gzip"));
        assert_eq!(server.header(&HOST), Some("localhost"));
    }

    #[test]
    fn rejects_an_http_11_request_without_a_host() {
        assert_rejects(Version::HTTP_11, "/", None, &RequestRejection::MissingHost);
    }

    #[test]
    fn serves_an_http_10_request_without_a_host() {
        assert_eq!(parse(Version::HTTP_10, "/", None).path(), "/");
    }

    #[test]
    fn rejects_a_host_that_is_not_an_authority() {
        assert_rejects(
            Version::HTTP_11,
            "/",
            Some("not a host"),
            &RequestRejection::MalformedHost {
                source: "not a host"
                    .parse::<Authority>()
                    .expect_err("the authority is malformed"),
            },
        );
    }

    #[test]
    fn rejects_an_absolute_form_target_that_disagrees_with_the_host_header() {
        assert_rejects(
            Version::HTTP_11,
            "http://elsewhere.example/admin",
            Some("localhost"),
            &RequestRejection::AuthorityMismatch,
        );
    }

    #[test]
    fn accepts_an_absolute_form_target_that_agrees_with_the_host_header() {
        assert_eq!(
            parse(
                Version::HTTP_11,
                "http://localhost/admin",
                Some("LOCALHOST")
            )
            .path(),
            "/admin"
        );
    }

    #[test]
    fn rejects_an_ambiguous_path() {
        assert_rejects(
            Version::HTTP_11,
            "/articles/../admin",
            Some("localhost"),
            &RequestRejection::DotSegmentInPath {
                segment: any_segment(),
            },
        );
    }

    #[test]
    fn rejects_a_repeated_singleton_header_before_reading_the_target() {
        let mut parts = parts_of(Version::HTTP_11, "/", Some("localhost"));
        parts
            .headers
            .append(HOST, HeaderValue::from_static("other"));

        assert_eq!(
            discriminant(
                &outcome_of(parts)
                    .err()
                    .expect("a repeated Host header is ambiguous")
            ),
            discriminant(&RequestRejection::RepeatedSingletonHeader {
                header: SingletonRequestHeader::Host,
            })
        );
    }

    #[test]
    fn carries_a_synthetic_request_without_headers() {
        let server = ServerParams::synthetic(Method::POST, "/articles".to_string());

        assert_eq!(server.method(), "POST");
        assert_eq!(server.path(), "/articles");
        assert_eq!(server.query_string(), "");
        assert_eq!(server.header(&HOST), None);
        assert_eq!(server.remote_addr(), unspecified_addr());
    }
}
