use uuid::Uuid;

use margaret::framework::macros::model;
use margaret::framework::model::column_default::ColumnDefault;

#[model(table = "uploads")]
pub struct Upload {
    #[column(primary_key, default = ColumnDefault::UuidV7)]
    pub id: Uuid,
    #[column]
    pub content: Vec<u8>,
}
