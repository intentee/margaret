use margaret::framework::macros::model;

#[model(table = "counters")]
#[derive(Clone, Debug, PartialEq)]
pub struct Counter {
    #[column(primary_key)]
    pub name: String,
    #[column]
    pub hits: i64,
}
