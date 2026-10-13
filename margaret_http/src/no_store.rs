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
    fn marks_the_response_no_store_and_no_cache_so_neither_http_11_nor_10_caches_keep_it() {
        let response = no_store(Response::text(200, ""));

        assert_eq!(response.header_value(&CACHE_CONTROL), Some("no-store"));
        assert_eq!(response.header_value(&PRAGMA), Some("no-cache"));
    }
}
