use margaret_http::response::Response;

pub(crate) fn frame_denied(response: Response) -> Response {
    response.header("content-security-policy", "frame-ancestors 'none'")
}
