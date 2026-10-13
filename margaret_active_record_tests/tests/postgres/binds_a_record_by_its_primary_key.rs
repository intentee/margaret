use margaret::framework::active_record::primary_key_binder::PrimaryKeyBinder;
use margaret_active_record_tests::models::author::Author;
use margaret_route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

use crate::postgres::created_author::created_author;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn binds_a_record_by_its_primary_key() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;

    assert!(matches!(
        PrimaryKeyBinder::<Author>::new(database)
            .bind(author.id.to_string())
            .await
            .expect("the author is bound"),
        RouteParameterBindingOutcome::Bound(bound) if bound == author
    ));
}
