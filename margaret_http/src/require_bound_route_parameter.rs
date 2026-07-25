use crate::http_route_parameter_binder::HttpRouteParameterBinder;
use crate::request::Request;
use crate::response::Response;

pub async fn require_bound_route_parameter<Binder>(
    request: &Request,
    name: &str,
    binder: &Binder,
) -> Result<Binder::Model, Response>
where
    Binder: HttpRouteParameterBinder,
{
    match request.path_param(name) {
        Some(value) => match binder.bind(value.to_string()).await {
            Ok(Some(model)) => Ok(model),
            Ok(None) => Err(Response::not_found()),
            Err(error) => {
                eprintln!(
                    "margaret_http: the route parameter binder for `{name}` failed: {error}"
                );

                Err(Response::text(500, "Internal Server Error"))
            }
        },
        None => Err(Response::not_found()),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::Infallible;
    use std::fmt;

    use async_trait::async_trait;
    use http::Method;

    use super::require_bound_route_parameter;
    use crate::http_route_parameter_binder::HttpRouteParameterBinder;
    use crate::request::Request;

    struct EvenNumberBinder;

    #[async_trait]
    impl HttpRouteParameterBinder for EvenNumberBinder {
        type Model = u32;
        type Error = Infallible;

        async fn bind(&self, value: String) -> Result<Option<u32>, Infallible> {
            Ok(value.parse::<u32>().ok().filter(|number| number % 2 == 0))
        }
    }

    struct FailingBinder;

    struct BinderFailure;

    impl fmt::Display for BinderFailure {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("the binder could not reach its backing store")
        }
    }

    #[async_trait]
    impl HttpRouteParameterBinder for FailingBinder {
        type Model = u32;
        type Error = BinderFailure;

        async fn bind(&self, _value: String) -> Result<Option<u32>, BinderFailure> {
            Err(BinderFailure)
        }
    }

    fn request_with_parameter(value: &str) -> Request {
        Request::new(Method::GET, "/numbers".to_string())
            .with_path_params(HashMap::from([("number".to_string(), value.to_string())]))
    }

    #[tokio::test]
    async fn binds_a_present_and_acceptable_route_parameter() {
        let request = request_with_parameter("42");

        assert_eq!(
            require_bound_route_parameter(&request, "number", &EvenNumberBinder)
                .await
                .ok(),
            Some(42)
        );
    }

    #[tokio::test]
    async fn reports_not_found_when_binding_rejects_the_value() {
        let request = request_with_parameter("7");

        assert_eq!(
            require_bound_route_parameter(&request, "number", &EvenNumberBinder)
                .await
                .err()
                .map(|response| response.status()),
            Some(404)
        );
    }

    #[tokio::test]
    async fn reports_not_found_when_the_route_parameter_is_absent() {
        let request = Request::new(Method::GET, "/numbers".to_string());

        assert_eq!(
            require_bound_route_parameter(&request, "number", &EvenNumberBinder)
                .await
                .err()
                .map(|response| response.status()),
            Some(404)
        );
    }

    #[tokio::test]
    async fn reports_a_server_error_when_the_binder_fails() {
        let request = request_with_parameter("42");

        assert_eq!(
            require_bound_route_parameter(&request, "number", &FailingBinder)
                .await
                .err()
                .map(|response| response.status()),
            Some(500)
        );
    }
}
