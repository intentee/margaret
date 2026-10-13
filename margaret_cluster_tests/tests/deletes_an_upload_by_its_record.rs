use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::deletion::Deletion;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_cluster_fixture::margaret::models::models_upload_upload::draft::Draft;
use margaret_cluster_fixture::margaret::schema::SCHEMA;
use margaret_cluster_fixture::models::upload::Upload;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn deletes_an_upload_by_its_record() {
    let started = StartedDatabase::with_schema(&SCHEMA).await;
    let database = started.database.as_ref();
    let upload = Upload::create(Draft {
        content: vec![1, 2, 3],
    })
    .run(database)
    .await
    .expect("the upload is stored");
    let id = upload.id;

    assert!(matches!(
        upload
            .delete(database)
            .await
            .expect("the upload is deleted"),
        Deletion::Deleted
    ));
    assert!(matches!(
        Upload::query()
            .id
            .eq(id)
            .find(database)
            .await
            .expect("the upload is looked up"),
        Lookup::Missing
    ));
}
