use chrono::DateTime;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::saving::Saving;
use margaret_cluster_fixture::margaret::models::models_token_acquisition_token_acquisition::draft::Draft;
use margaret_cluster_fixture::margaret::schema::SCHEMA;
use margaret_cluster_fixture::models::token_acquisition::TokenAcquisition;
use margaret_cluster_fixture::models::token_acquisition_outcome::TokenAcquisitionOutcome;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn saves_a_token_acquisition_by_its_record() {
    let started = StartedDatabase::with_schema(&SCHEMA).await;
    let database = started.database.as_ref();
    let mut acquisition = TokenAcquisition::create(Draft {
        attempted_at: DateTime::UNIX_EPOCH,
        outcome: TokenAcquisitionOutcome::Unavailable,
    })
    .run(database)
    .await
    .expect("the acquisition is recorded");

    acquisition.outcome = TokenAcquisitionOutcome::Refused;

    assert!(matches!(
        acquisition
            .save(database)
            .await
            .expect("the acquisition is saved"),
        Saving::Saved
    ));
    assert!(matches!(
        TokenAcquisition::query()
            .id
            .eq(acquisition.id)
            .find(database)
            .await
            .expect("the acquisition is looked up"),
        Lookup::Found(TokenAcquisition {
            outcome: TokenAcquisitionOutcome::Refused,
            ..
        })
    ));
}
