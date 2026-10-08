use uuid::Uuid;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::author::Author;

use crate::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_missing_record() {
    let started = started_with_models().await;

    assert_eq!(
        Author::query()
            .id
            .eq(Uuid::nil())
            .find(started.database.as_ref())
            .await
            .expect("the absent author is read"),
        Lookup::Missing
    );
}
