use margaret::framework::macros::model;

#[model(table = "fragment_metadata")]
pub struct FragmentMetadata {
    #[column(primary_key)]
    pub partition: uuid::Uuid,
    #[column(primary_key, byte_length = 32)]
    pub hash: Vec<u8>,
    #[column(minimum = 0)]
    pub size_payload: i64,
}
