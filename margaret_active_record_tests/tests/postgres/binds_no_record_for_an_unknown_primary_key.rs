use margaret::framework::active_record::primary_key_binder::PrimaryKeyBinder;
use margaret_route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;
use uuid::Uuid;

use margaret_active_record_tests::models::author::Author;

use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn binds_no_record_for_an_unknown_primary_key() {
    let started = started_with_models().await;

    assert!(matches!(
        PrimaryKeyBinder::<Author>::new(&started.database)
            .bind(Uuid::nil().to_string())
            .await
            .expect("the lookup completes"),
        RouteParameterBindingOutcome::NotFound
    ));
}
