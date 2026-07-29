use crate::request::Request;
use crate::response::Response;

/// # Errors
///
/// Returns `Response` propagated from the work it performs.
pub fn require_route_parameter(request: &Request, name: &str) -> Result<String, Response> {
    match request.path_param(name) {
        Some(value) => Ok(value.to_string()),
        None => Err(Response::not_found()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use http::Method;

    use super::require_route_parameter;
    use crate::request::Request;

    #[test]
    fn returns_a_present_route_parameter() {
        let request = Request::new(Method::GET, "/articles/42".to_string())
            .with_path_params(BTreeMap::from([("article".to_string(), "42".to_string())]));

        assert_eq!(
            require_route_parameter(&request, "article").ok(),
            Some("42".to_string())
        );
    }

    #[test]
    fn reports_not_found_for_an_absent_route_parameter() {
        let request = Request::new(Method::GET, "/articles".to_string());

        assert_eq!(
            require_route_parameter(&request, "article")
                .err()
                .map(|response| response.status()),
            Some(404)
        );
    }
}
