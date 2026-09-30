use serde_json::json;

use margaret_oauth_vocabulary::client_id::ClientId;

#[test]
fn refuses_a_client_id_that_is_not_a_json_string() {
    assert!(serde_json::from_value::<ClientId>(json!(7)).is_err());
    assert!(serde_json::from_value::<ClientId>(json!("")).is_err());
}
