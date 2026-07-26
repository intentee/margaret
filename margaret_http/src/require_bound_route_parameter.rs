use crate::http_route_parameter_binder::HttpRouteParameterBinder;
use crate::request::Request;
use crate::route_parameter_outcome::RouteParameterOutcome;
use crate::route_parameter_rejection::RouteParameterRejection;

pub async fn require_bound_route_parameter<Binder>(
    request: &Request,
    name: &str,
    binder: &Binder,
) -> Result<Binder::Model, RouteParameterRejection>
where
    Binder: HttpRouteParameterBinder,
{
    let Some(value) = request.path_param(name) else {
        return Err(RouteParameterRejection::NotFound);
    };

    match binder.bind(value.to_string()).await {
        Ok(RouteParameterOutcome::Found(model)) => Ok(model),
        Ok(RouteParameterOutcome::NotFound) => Err(RouteParameterRejection::NotFound),
        Err(error) => Err(RouteParameterRejection::SystemError(error)),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use async_trait::async_trait;
    use http::Method;

    use super::require_bound_route_parameter;
    use crate::http_route_parameter_binder::HttpRouteParameterBinder;
    use crate::request::Request;
    use crate::route_parameter_outcome::RouteParameterOutcome;
    use crate::route_parameter_rejection_response::route_parameter_rejection_response;

    struct EvenNumberBinder;

    #[async_trait]
    impl HttpRouteParameterBinder for EvenNumberBinder {
        type Model = u32;

        async fn bind(&self, value: String) -> anyhow::Result<RouteParameterOutcome<u32>> {
            Ok(match value.parse::<u32>().ok().filter(|number| number % 2 == 0) {
                Some(number) => RouteParameterOutcome::Found(number),
                None => RouteParameterOutcome::NotFound,
            })
        }
    }

    struct FailingBinder;

    #[async_trait]
    impl HttpRouteParameterBinder for FailingBinder {
        type Model = u32;

        async fn bind(&self, _value: String) -> anyhow::Result<RouteParameterOutcome<u32>> {
            Err(anyhow::anyhow!("the datastore is unavailable"))
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
    async fn rejects_as_not_found_when_binding_rejects_the_value() {
        let request = request_with_parameter("7");
        let rejection = require_bound_route_parameter(&request, "number", &EvenNumberBinder)
            .await
            .expect_err("the odd value is rejected");

        assert_eq!(
            route_parameter_rejection_response([Some(rejection)]).status(),
            404
        );
    }

    #[tokio::test]
    async fn rejects_as_not_found_when_the_route_parameter_is_absent() {
        let request = Request::new(Method::GET, "/numbers".to_string());
        let rejection = require_bound_route_parameter(&request, "number", &EvenNumberBinder)
            .await
            .expect_err("the absent parameter is rejected");

        assert_eq!(
            route_parameter_rejection_response([Some(rejection)]).status(),
            404
        );
    }

    #[tokio::test]
    async fn rejects_as_a_system_error_when_the_binder_fails() {
        let request = request_with_parameter("42");
        let rejection = require_bound_route_parameter(&request, "number", &FailingBinder)
            .await
            .expect_err("the failing binder is rejected");

        assert_eq!(
            route_parameter_rejection_response([Some(rejection)]).status(),
            500
        );
    }

    struct BarrierBinder {
        barrier: std::sync::Arc<tokio::sync::Barrier>,
    }

    #[async_trait]
    impl HttpRouteParameterBinder for BarrierBinder {
        type Model = String;

        async fn bind(&self, value: String) -> anyhow::Result<RouteParameterOutcome<String>> {
            self.barrier.wait().await;

            Ok(RouteParameterOutcome::Found(value))
        }
    }

    #[tokio::test]
    async fn resolves_two_binders_concurrently() {
        let barrier = std::sync::Arc::new(tokio::sync::Barrier::new(2));
        let first = BarrierBinder {
            barrier: barrier.clone(),
        };
        let second = BarrierBinder { barrier };
        let request = Request::new(Method::GET, "/pair".to_string()).with_path_params(
            HashMap::from([
                ("first".to_string(), "alpha".to_string()),
                ("second".to_string(), "beta".to_string()),
            ]),
        );

        let (first_outcome, second_outcome) = crate::join!(
            require_bound_route_parameter(&request, "first", &first),
            require_bound_route_parameter(&request, "second", &second),
        );

        assert_eq!(first_outcome.ok(), Some("alpha".to_string()));
        assert_eq!(second_outcome.ok(), Some("beta".to_string()));
    }
}
