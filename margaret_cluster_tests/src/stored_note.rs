use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct StoredNote {
    pub body: String,
    pub id: Uuid,
}
