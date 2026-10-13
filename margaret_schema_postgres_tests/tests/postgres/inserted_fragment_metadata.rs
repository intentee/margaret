use uuid::Uuid;

use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_schema_postgres_fixture::fragment_metadata::FragmentMetadata;

pub async fn inserted_fragment_metadata(
    database: &Database,
    partition: Uuid,
    hash: Vec<u8>,
) -> FragmentMetadata {
    let metadata = FragmentMetadata {
        partition,
        hash,
        size_payload: 0,
    };

    metadata
        .insert()
        .run(database)
        .await
        .expect("the fragment metadata is inserted");

    metadata
}
