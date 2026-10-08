use margaret_macros::model;

#[model(table = "client_assertions")]
#[primary_key(columns = [client_id, assertion])]
pub struct ClientAssertionRecord {
    #[column]
    pub client_id: String,
    #[column(byte_length = 32)]
    pub assertion: Vec<u8>,
    #[column]
    #[index]
    pub expires_at: i64,
}
