use uuid::Uuid;

use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret::framework::database::database::Database;
use margaret_schema_postgres_fixture::fragment::Fragment;
use margaret_schema_postgres_fixture::fragment_metadata::FragmentMetadata;
use margaret_schema_postgres_fixture::margaret::models::fragment_metadata_fragment_metadata::primary_key::PrimaryKey;

use crate::inserted_fragment_metadata::inserted_fragment_metadata;
use crate::started_with_fixture::started_with_fixture;

async fn insert_fragment(
    database: &Database,
    metadata: Key<FragmentMetadata>,
) -> Result<(), ActiveRecordError> {
    Fragment {
        metadata,
        context: Uuid::new_v4(),
        slot: 0,
    }
    .insert()
    .run(database)
    .await
}

#[tokio::test]
async fn a_fragment_with_matching_metadata_is_accepted() {
    let started = started_with_fixture().await;
    let database = started.database.as_ref();
    let metadata = inserted_fragment_metadata(database, Uuid::new_v4(), vec![7u8; 32]).await;

    insert_fragment(database, Key::of(&metadata))
        .await
        .expect("a fragment backed by its metadata is accepted");
}

#[tokio::test]
async fn a_fragment_whose_hash_has_no_metadata_is_rejected() {
    let started = started_with_fixture().await;
    let database = started.database.as_ref();
    let metadata = inserted_fragment_metadata(database, Uuid::new_v4(), vec![7u8; 32]).await;

    assert!(matches!(
        insert_fragment(
            database,
            Key::new(PrimaryKey {
                partition: metadata.partition,
                hash: vec![9u8; 32],
            }),
        )
        .await,
        Err(ActiveRecordError::ForeignKeyViolation {
            statement: StatementKind::Insert,
            table: "fragment",
            ..
        })
    ));
}

#[tokio::test]
async fn a_fragment_whose_partition_has_no_metadata_is_rejected() {
    let started = started_with_fixture().await;
    let database = started.database.as_ref();
    let metadata = inserted_fragment_metadata(database, Uuid::new_v4(), vec![7u8; 32]).await;

    assert!(matches!(
        insert_fragment(
            database,
            Key::new(PrimaryKey {
                partition: Uuid::new_v4(),
                hash: metadata.hash,
            }),
        )
        .await,
        Err(ActiveRecordError::ForeignKeyViolation {
            statement: StatementKind::Insert,
            table: "fragment",
            ..
        })
    ));
}
