use http::HeaderMap;
use http::HeaderValue;
use http::header::CACHE_CONTROL;
use http::header::CONTENT_SECURITY_POLICY;
use http::header::REFERRER_POLICY;
use http::header::STRICT_TRANSPORT_SECURITY;
use http::header::X_CONTENT_TYPE_OPTIONS;
use http::header::X_FRAME_OPTIONS;

pub(crate) fn apply_security_headers(headers: &mut HeaderMap) {
    headers.insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(
            "default-src 'none'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'",
        ),
    );
    headers.insert(
        STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=63072000; includeSubDomains"),
    );
    headers.insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    headers.insert(X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(REFERRER_POLICY, HeaderValue::from_static("no-referrer"));
    headers.insert(
        http::header::HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("accelerometer=(), camera=(), geolocation=(), microphone=()"),
    );
    headers.insert(
        http::header::HeaderName::from_static("cross-origin-opener-policy"),
        HeaderValue::from_static("same-origin"),
    );
    headers.insert(
        http::header::HeaderName::from_static("cross-origin-resource-policy"),
        HeaderValue::from_static("same-origin"),
    );
    headers
        .entry(CACHE_CONTROL)
        .or_insert(HeaderValue::from_static("no-store"));
}
