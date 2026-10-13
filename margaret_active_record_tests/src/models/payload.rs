use std::collections::BTreeMap;

use uuid::Uuid;

use margaret::framework::active_record::json::Json;
use margaret::framework::macros::model;
use margaret::framework::model::column_default::ColumnDefault;

#[model(table = "payloads")]
#[index(name = "payloads_by_document", fields = [document, id])]
#[derive(Clone, Debug, PartialEq)]
pub struct Payload {
    #[column(primary_key, default = ColumnDefault::UuidV7)]
    pub id: Uuid,
    #[column]
    pub document: Json<BTreeMap<Vec<u8>, i64>>,
}
