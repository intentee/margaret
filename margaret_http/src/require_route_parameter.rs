use crate::request::Request;
use crate::response::Response;
use crate::route_parameter_binding_outcome::RouteParameterBindingOutcome;

fn route_parameter_value<Value>(value: String) -> RouteParameterBindingOutcome<Value>
where
    Value: TryFrom<String>,
{
    match Value::try_from(value) {
        Ok(value) => RouteParameterBindingOutcome::Bound(value),
        Err(_) => RouteParameterBindingOutcome::NotFound,
    }
}

/// # Errors
///
/// Returns `Response` when the request does not carry a usable value for the parameter.
pub fn require_route_parameter<Value>(request: &Request, name: &str) -> Result<Value, Response>
where
    Value: TryFrom<String>,
{
    match request.path_param(name) {
        Some(value) => match route_parameter_value(value.to_string()) {
            RouteParameterBindingOutcome::Bound(value) => Ok(value),
            RouteParameterBindingOutcome::NotFound => Err(Response::not_found()),
        },
        None => Err(Response::not_found()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::num::ParseIntError;

    use http::Method;

    use super::require_route_parameter;
    use crate::request::Request;

    #[derive(Debug, Eq, PartialEq)]
    struct ArticleSlug(String);

    #[derive(Debug, Eq, PartialEq)]
    struct PublishedYear(u16);

    impl From<String> for ArticleSlug {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    impl TryFrom<String> for PublishedYear {
        type Error = ParseIntError;

        fn try_from(value: String) -> Result<Self, ParseIntError> {
            value.parse().map(Self)
        }
    }

    fn request_with_parameter(value: &str) -> Request {
        Request::new(Method::GET, "/articles/42".to_string())
            .with_path_params(HashMap::from([("article".to_string(), value.to_string())]))
    }

    #[test]
    fn returns_a_present_route_parameter() {
        let request = request_with_parameter("42");

        assert_eq!(
            require_route_parameter(&request, "article").ok(),
            Some("42".to_string())
        );
    }

    #[test]
    fn reports_not_found_for_an_absent_route_parameter() {
        let request = Request::new(Method::GET, "/articles".to_string());

        assert_eq!(
            require_route_parameter::<String>(&request, "article")
                .err()
                .map(|response| response.status()),
            Some(404)
        );
    }

    #[test]
    fn converts_a_route_parameter_into_a_value_type() {
        let request = request_with_parameter("shipping-margaret");

        assert_eq!(
            require_route_parameter(&request, "article").ok(),
            Some(ArticleSlug("shipping-margaret".to_string()))
        );
    }

    #[test]
    fn reports_not_found_when_a_value_type_rejects_the_route_parameter() {
        let request = request_with_parameter("not-a-year");

        assert_eq!(
            require_route_parameter::<PublishedYear>(&request, "article")
                .err()
                .map(|response| response.status()),
            Some(404)
        );
    }
}
