use margaret_http::response::Response;

pub(crate) fn no_store(response: Response) -> Response {
    response
        .header("cache-control", "no-store")
        .header("pragma", "no-cache")
}
