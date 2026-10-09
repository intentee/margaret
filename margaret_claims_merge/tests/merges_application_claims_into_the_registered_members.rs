use serde_json::Map;
use serde_json::Value;
use serde_json::json;

use margaret_claims_merge::merge_claims::merge_claims;

#[test]
fn merges_application_claims_into_the_registered_members() {
    let members = Map::from_iter([("iss".to_string(), json!("https://issuer.example"))]);

    assert_eq!(
        Value::Object(merge_claims(members, &json!({ "name": "demo" })).expect("the claims merge")),
        json!({
            "iss": "https://issuer.example",
            "name": "demo",
        })
    );
}
