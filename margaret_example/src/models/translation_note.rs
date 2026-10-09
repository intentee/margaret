use uuid::Uuid;

use margaret::framework::macros::model;
use margaret::framework::model::on_delete::OnDelete;

use crate::models::article_translation::ArticleTranslation;

#[model(table = "translation_notes")]
#[derive(Clone)]
pub struct TranslationNote {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    pub body: String,
    #[column]
    #[foreign_key(on_delete = OnDelete::Cascade)]
    #[index]
    pub source: ArticleTranslation,
}
