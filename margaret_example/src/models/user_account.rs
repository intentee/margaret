use uuid::Uuid;

use margaret::framework::macros::model;

#[model(table = "users")]
pub struct UserAccount {
    #[column(primary_key)]
    pub id: Uuid,
    #[column(unique)]
    pub name: String,
}
