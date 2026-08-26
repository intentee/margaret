use margaret::framework::macros::model;

#[model(table = "translation_reviews")]
#[primary_key(columns = [article_id, locale, reviewer])]
#[unique(columns = [content_hash, reviewer])]
#[index(name = "translation_reviews_reviewer_score", columns = [reviewer, score])]
#[foreign_key(
    columns = [article_id, locale],
    references = crate::models::article_translation::ArticleTranslation,
    on_delete = cascade
)]
#[derive(Clone)]
pub struct TranslationReview {
    #[column]
    pub article_id: uuid::Uuid,
    #[column]
    pub locale: String,
    #[column]
    pub reviewer: String,
    #[column(byte_length = 32)]
    pub content_hash: Vec<u8>,
    #[column(minimum = 0)]
    pub score: i32,
}
