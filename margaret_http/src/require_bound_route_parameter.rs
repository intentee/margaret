use crate::http_route_parameter_binder::HttpRouteParameterBinder;
use crate::request::Request;
use crate::route_parameter_outcome::RouteParameterOutcome;

pub async fn require_bound_route_parameter<Binder>(
    request: &Request,
    name: &str,
    binder: &Binder,
) -> anyhow::Result<RouteParameterOutcome<Binder::Model>>
where
    Binder: HttpRouteParameterBinder,
{
    let Some(value) = request.path_param(name) else {
        return Ok(RouteParameterOutcome::NotFound);
    };

    binder.bind(value.to_string()).await
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::future::pending;

    use async_trait::async_trait;
    use http::Method;

    use super::require_bound_route_parameter;
    use crate::http_route_parameter_binder::HttpRouteParameterBinder;
    use crate::request::Request;
    use crate::route_parameter_outcome::RouteParameterOutcome;

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

        let outcome = require_bound_route_parameter(&request, "number", &EvenNumberBinder)
            .await
            .expect("the acceptable value binds without a system error");

        assert_eq!(outcome, RouteParameterOutcome::Found(42));
    }

    #[tokio::test]
    async fn resolves_to_not_found_when_binding_rejects_the_value() {
        let request = request_with_parameter("7");

        let outcome = require_bound_route_parameter(&request, "number", &EvenNumberBinder)
            .await
            .expect("the rejected value is a not-found outcome, not a system error");

        assert_eq!(outcome, RouteParameterOutcome::NotFound);
    }

    #[tokio::test]
    async fn resolves_to_not_found_when_the_route_parameter_is_absent() {
        let request = Request::new(Method::GET, "/numbers".to_string());

        let outcome = require_bound_route_parameter(&request, "number", &EvenNumberBinder)
            .await
            .expect("the absent parameter is a not-found outcome, not a system error");

        assert_eq!(outcome, RouteParameterOutcome::NotFound);
    }

    #[tokio::test]
    async fn propagates_a_system_error_when_the_binder_fails() {
        let request = request_with_parameter("42");

        let error = require_bound_route_parameter(&request, "number", &FailingBinder)
            .await
            .expect_err("a binder failure surfaces as a system error, never a not-found");

        assert_eq!(error.to_string(), "the datastore is unavailable");
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

        let (first_outcome, second_outcome) = crate::try_join!(
            require_bound_route_parameter(&request, "first", &first),
            require_bound_route_parameter(&request, "second", &second),
        )
        .expect("both barrier binders resolve without a system error");

        assert_eq!(first_outcome, RouteParameterOutcome::Found("alpha".to_string()));
        assert_eq!(second_outcome, RouteParameterOutcome::Found("beta".to_string()));
    }

    #[tokio::test]
    async fn a_system_error_short_circuits_a_pending_binder() {
        let request = request_with_parameter("42");

        let outcome = crate::try_join!(
            require_bound_route_parameter(&request, "number", &FailingBinder),
            pending::<anyhow::Result<RouteParameterOutcome<u32>>>(),
        );

        let error = outcome.expect_err("the failing binder short-circuits the pending future");

        assert_eq!(error.to_string(), "the datastore is unavailable");
    }
}
