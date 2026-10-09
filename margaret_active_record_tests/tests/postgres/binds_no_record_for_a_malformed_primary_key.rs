use margaret::framework::active_record::primary_key_binder::PrimaryKeyBinder;
use margaret_active_record_tests::models::author::Author;
use margaret_route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn binds_no_record_for_a_malformed_primary_key() {
    let started = started_with_models().await;

    assert!(matches!(
        PrimaryKeyBinder::<Author>::new(&started.database)
            .bind("not-a-uuid".to_string())
            .await
            .expect("a malformed key binds nothing"),
        RouteParameterBindingOutcome::NotFound
    ));
}
