use chrono::DateTime;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_cluster_fixture::margaret::models::models_token_acquisition_token_acquisition::draft::Draft;
use margaret_cluster_fixture::margaret::schema::SCHEMA;
use margaret_cluster_fixture::models::token_acquisition::TokenAcquisition;
use margaret_cluster_fixture::models::token_acquisition_outcome::TokenAcquisitionOutcome;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn updates_the_token_acquisitions_of_an_outcome() {
    let started = StartedDatabase::with_schema(&SCHEMA).await;
    let database = started.database.as_ref();
    let refused = TokenAcquisition::create(Draft {
        attempted_at: DateTime::UNIX_EPOCH,
        outcome: TokenAcquisitionOutcome::Refused,
    })
    .run(database)
    .await
    .expect("the refused acquisition is recorded");

    TokenAcquisition::create(Draft {
        attempted_at: DateTime::UNIX_EPOCH,
        outcome: TokenAcquisitionOutcome::Acquired,
    })
    .run(database)
    .await
    .expect("the acquired acquisition is recorded");

    assert_eq!(
        TokenAcquisition::query()
            .outcome
            .eq(TokenAcquisitionOutcome::Refused)
            .update(database, |columns| {
                columns.outcome.to(TokenAcquisitionOutcome::Unavailable)
            })
            .await
            .expect("the refused acquisitions are updated"),
        1
    );
    assert!(matches!(
        TokenAcquisition::query()
            .id
            .eq(refused.id)
            .find(database)
            .await
            .expect("the refused acquisition is looked up"),
        Lookup::Found(TokenAcquisition {
            outcome: TokenAcquisitionOutcome::Unavailable,
            ..
        })
    ));
}
