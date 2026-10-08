use uuid::Uuid;

use margaret::framework::macros::model;

#[model(table = "uploads")]
pub struct Upload {
    #[column(primary_key)]
    pub id: Uuid,
    #[column]
    pub content: Vec<u8>,
}
