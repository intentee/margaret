use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret_schema_postgres_fixture::fragment::Fragment;

use crate::inserted_fragment_metadata::inserted_fragment_metadata;
use crate::started_with_fixture::started_with_fixture;

#[tokio::test]
async fn a_second_fragment_sharing_a_context_and_slot_is_rejected() {
    let started = started_with_fixture().await;
    let database = started.database.as_ref();
    let hash = vec![5u8; 32];
    let context = Uuid::new_v4();
    let first = inserted_fragment_metadata(database, Uuid::new_v4(), hash.clone()).await;
    let second = inserted_fragment_metadata(database, Uuid::new_v4(), hash).await;

    Fragment {
        metadata: Key::of(&first),
        context,
        slot: 1,
    }
    .insert()
    .run(database)
    .await
    .expect("the first fragment is accepted");

    assert!(matches!(
        Fragment {
            metadata: Key::of(&second),
            context,
            slot: 1,
        }
        .insert()
        .run(database)
        .await,
        Err(ActiveRecordError::UniqueViolation {
            statement: StatementKind::Insert,
            table: "fragment",
            ..
        })
    ));
}
