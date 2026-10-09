use margaret::framework::macros::model;

#[model(table = "key_sets")]
#[derive(Clone, Debug, PartialEq)]
pub struct KeySet {
    #[column(primary_key)]
    pub name: String,
    #[column(minimum = 0)]
    pub generation: i64,
    #[column]
    pub document: String,
}
