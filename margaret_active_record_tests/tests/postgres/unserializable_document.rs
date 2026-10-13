use std::collections::BTreeMap;

use margaret::framework::active_record::json::Json;

pub fn unserializable_document() -> Json<BTreeMap<Vec<u8>, i64>> {
    let mut document = BTreeMap::new();

    document.insert(vec![1_u8], 1_i64);

    Json::new(document)
}
