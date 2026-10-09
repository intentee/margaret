use margaret::framework::active_record::key::Key;
use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

use crate::models::article_translation::ArticleTranslation;

#[model(table = "translation_reviews")]
#[primary_key(fields = [translation, reviewer])]
#[unique(fields = [content_hash, reviewer])]
#[index(name = "translation_reviews_reviewer_score", fields = [reviewer, score])]
#[derive(Clone)]
pub struct TranslationReview {
    #[column]
    #[foreign_key(on_delete = OnDelete::Cascade)]
    pub translation: Key<ArticleTranslation>,
    #[column]
    pub reviewer: String,
    #[column(byte_length = 32)]
    pub content_hash: Vec<u8>,
    #[column(minimum = 0)]
    pub score: i32,
}
