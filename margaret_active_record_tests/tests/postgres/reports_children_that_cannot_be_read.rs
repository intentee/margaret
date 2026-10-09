use margaret::framework::active_record::active_record_error::ActiveRecordError;
use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::statement_kind::StatementKind;
use margaret::framework::sql_identifier::table_namespace::TableNamespace;
use margaret_active_record_tests::models::author::Author;
use margaret_active_record_tests::models::author_with_articles::AuthorWithArticles;
use margaret_database_tests::table_privilege::TablePrivilege;

use crate::postgres::created_article::created_article;
use crate::postgres::created_author::created_author;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn reports_children_that_cannot_be_read() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;

    created_article(database, &author, "Shipping").await;
    started
        .administration
        .revoke(
            TablePrivilege::Select,
            TableNamespace::Application,
            "article_translations",
        )
        .await;

    assert!(matches!(
        Author::query()
            .id
            .eq(author.id)
            .load::<AuthorWithArticles, _>(database)
            .await,
        Err(ActiveRecordError::Database {
            statement: StatementKind::Select,
            table: "article_translations",
            ..
        })
    ));
}
