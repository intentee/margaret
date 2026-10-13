use margaret_http::response::Response;
use margaret_http_tests::redirection::Redirection;

#[must_use]
pub fn issued_code(redirect: &Response) -> String {
    Redirection::of(redirect).parameter("code").to_string()
}
