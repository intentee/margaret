use margaret_macros::model;

#[model(table = "signing_key_sets")]
pub struct SigningKeySet {
    #[column(primary_key)]
    pub name: String,
    #[column]
    pub generation: i64,
    #[column]
    pub document: String,
}
