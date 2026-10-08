use margaret::framework::active_record::primary_key_binder::PrimaryKeyBinder;
use margaret_route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_active_record_tests::models::author::Author;
use margaret_database_tests::table_privilege::TablePrivilege;

use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_binding_the_database_refuses() {
    let started = started_with_models().await;

    started
        .administration
        .revoke(
            TablePrivilege::Select,
            TableNamespace::Application,
            "authors",
        )
        .await;

    let error = PrimaryKeyBinder::<Author>::new(&started.database)
        .bind(Uuid::nil().to_string())
        .await
        .err()
        .expect("the refused lookup is reported");

    assert!(matches!(
        error.downcast_ref::<ActiveRecordError>(),
        Some(ActiveRecordError::Database {
            statement: StatementKind::Select,
            table: "authors",
            ..
        })
    ));
}
