use crate::request::Request;
use crate::response::Response;

pub fn require_route_parameter(request: &Request, name: &str) -> Result<String, Response> {
    match request.path_param(name) {
        Some(value) => Ok(value.to_string()),
        None => Err(Response::not_found()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use http::Method;
    use tokio_util::sync::CancellationToken;

    use super::require_route_parameter;
    use crate::request::Request;

    #[test]
    fn returns_a_present_route_parameter() {
        let request = Request::new(
            Method::GET,
            "/articles/42".to_string(),
            CancellationToken::new(),
        )
        .with_path_params(HashMap::from([("article".to_string(), "42".to_string())]));

        assert_eq!(
            require_route_parameter(&request, "article").ok(),
            Some("42".to_string())
        );
    }

    #[test]
    fn reports_not_found_for_an_absent_route_parameter() {
        let request = Request::new(
            Method::GET,
            "/articles".to_string(),
            CancellationToken::new(),
        );

        assert_eq!(
            require_route_parameter(&request, "article")
                .err()
                .map(|response| response.status()),
            Some(404)
        );
    }
}
