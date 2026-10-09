use uuid::Uuid;

use margaret::framework::macros::model;

#[model(table = "fragment_metadata")]
#[primary_key(columns = [partition, hash])]
pub struct FragmentMetadata {
    #[column]
    pub partition: Uuid,
    #[column(byte_length = 32)]
    pub hash: Vec<u8>,
    #[column(minimum = 0)]
    pub size_payload: i64,
}
