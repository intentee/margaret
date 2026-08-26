use margaret::framework::macros::model;

use crate::models::article_translation::ArticleTranslation;

#[model(table = "translation_notes")]
#[derive(Clone)]
pub struct TranslationNote {
    #[column(primary_key)]
    pub id: uuid::Uuid,
    #[column]
    pub body: String,
    #[column]
    #[foreign_key(on_delete = cascade)]
    #[index]
    pub source: ArticleTranslation,
}
