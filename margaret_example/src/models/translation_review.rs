use margaret::framework::macros::model;

#[model(table = "translation_reviews")]
#[foreign_key(
    columns = [article_id, locale],
    references = crate::models::article_translation::ArticleTranslation,
    on_delete = cascade
)]
#[derive(Clone)]
pub struct TranslationReview {
    #[column(primary_key)]
    pub article_id: uuid::Uuid,
    #[column(primary_key)]
    pub locale: String,
    #[column(primary_key)]
    pub reviewer: String,
    #[column(byte_length = 32)]
    pub content_hash: Vec<u8>,
    #[column(minimum = 0)]
    pub score: i32,
}
