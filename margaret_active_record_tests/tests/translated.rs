use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_translation::ArticleTranslation;

pub async fn translated(
    database: &Database,
    article: &Article,
    locale: &str,
) -> ArticleTranslation {
    let translation = ArticleTranslation {
        article: Key::of(article),
        locale: locale.to_string(),
        title: format!("{} ({locale})", article.title),
    };

    translation
        .insert()
        .run(database)
        .await
        .expect("the translation is inserted");

    translation
}
