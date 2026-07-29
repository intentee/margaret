use crate::http_route_parameter_binder::HttpRouteParameterBinder;
use crate::request::Request;
use crate::request_binding_error::RequestBindingError;
use crate::route_parameter_binding_outcome::RouteParameterBindingOutcome;

/// # Errors
///
/// Returns `RequestBindingError::RouteParameterBinder`.
pub async fn require_bound_route_parameter<Binder>(
    request: &Request,
    name: &'static str,
    binder: &Binder,
) -> Result<RouteParameterBindingOutcome<Binder::Model>, RequestBindingError>
where
    Binder: HttpRouteParameterBinder,
{
    match request.path_param(name) {
        Some(value) => binder.bind(value.to_string()).await.map_err(|source| {
            RequestBindingError::RouteParameterBinder {
                parameter: name,
                source,
            }
        }),
        None => Ok(RouteParameterBindingOutcome::NotFound),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use async_trait::async_trait;
    use http::Method;

    use super::require_bound_route_parameter;
    use crate::http_route_parameter_binder::HttpRouteParameterBinder;
    use crate::request::Request;
    use crate::route_parameter_binding_outcome::RouteParameterBindingOutcome;

    struct EvenNumberBinder;
    struct FailingBinder;

    #[async_trait]
    impl HttpRouteParameterBinder for EvenNumberBinder {
        type Model = u32;

        async fn bind(&self, value: String) -> anyhow::Result<RouteParameterBindingOutcome<u32>> {
            Ok(
                match value.parse::<u32>().ok().filter(|number| number % 2 == 0) {
                    Some(number) => RouteParameterBindingOutcome::Bound(number),
                    None => RouteParameterBindingOutcome::NotFound,
                },
            )
        }
    }

    #[async_trait]
    impl HttpRouteParameterBinder for FailingBinder {
        type Model = u32;

        async fn bind(&self, _value: String) -> anyhow::Result<RouteParameterBindingOutcome<u32>> {
            anyhow::bail!("database unavailable")
        }
    }

    fn request_with_parameter(value: &str) -> Request {
        Request::new(Method::GET, "/numbers".to_string())
            .with_path_params(BTreeMap::from([("number".to_string(), value.to_string())]))
    }

    async fn bound_number(request: &Request) -> Option<u32> {
        match require_bound_route_parameter(request, "number", &EvenNumberBinder)
            .await
            .expect("the binder must succeed")
        {
            RouteParameterBindingOutcome::Bound(number) => Some(number),
            RouteParameterBindingOutcome::NotFound => None,
        }
    }

    #[tokio::test]
    async fn binds_a_present_and_acceptable_route_parameter() {
        let request = request_with_parameter("42");

        assert_eq!(bound_number(&request).await, Some(42));
    }

    #[tokio::test]
    async fn reports_not_found_when_binding_rejects_the_value() {
        let request = request_with_parameter("7");

        assert_eq!(bound_number(&request).await, None);
    }

    #[tokio::test]
    async fn reports_not_found_when_the_route_parameter_is_absent() {
        let request = Request::new(Method::GET, "/numbers".to_string());

        assert_eq!(bound_number(&request).await, None);
    }

    #[tokio::test]
    async fn preserves_a_binder_failure_as_a_system_error() {
        let request = request_with_parameter("42");

        let error = require_bound_route_parameter(&request, "number", &FailingBinder)
            .await
            .err()
            .expect("the binder must fail");

        assert_eq!(
            error.to_string(),
            "the binder for route parameter 'number' failed: database unavailable"
        );
        assert_eq!(
            std::error::Error::source(&error).map(ToString::to_string),
            Some("database unavailable".to_string())
        );
    }
}
