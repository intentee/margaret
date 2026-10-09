use crate::response::Response;

#[must_use]
pub fn no_store(response: Response) -> Response {
    response
        .header("cache-control", "no-store")
        .header("pragma", "no-cache")
}

#[cfg(test)]
mod tests {
    use http::header::CACHE_CONTROL;
    use http::header::PRAGMA;

    use super::no_store;
    use crate::response::Response;

    #[test]
    fn forbids_caching_the_response() {
        let response = no_store(Response::text(200, ""));

        assert_eq!(response.header_value(&CACHE_CONTROL), Some("no-store"));
        assert_eq!(response.header_value(&PRAGMA), Some("no-cache"));
    }
}
