use serde_json::json;

use margaret_oauth_vocabulary::client_id::ClientId;

#[test]
fn reads_a_client_id_from_a_json_string() {
    let client_id: ClientId =
        serde_json::from_value(json!("portal")).expect("the client identifier is visible");

    assert_eq!(client_id.as_str(), "portal");
    assert_eq!(
        serde_json::to_value(&client_id).expect("the client identifier serializes"),
        json!("portal")
    );
}
