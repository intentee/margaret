use margaret::framework::macros::model;

#[model(table = "measurements")]
#[derive(Clone, Debug, PartialEq)]
pub struct Measurement {
    #[column(primary_key)]
    pub id: i64,
    #[column]
    pub small: i32,
    #[column]
    pub ratio: f32,
    #[column]
    pub precise: f64,
    #[column]
    pub flag: bool,
}
