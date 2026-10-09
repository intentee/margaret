use serde_json::Map;
use serde_json::json;

use margaret_claims_merge::merge_claims::merge_claims;

#[test]
fn reports_application_claims_colliding_with_a_registered_member() {
    let members = Map::from_iter([("sub".to_string(), json!("subject"))]);

    assert_eq!(
        merge_claims(members, &json!({ "sub": "impostor" }))
            .expect_err("the claims collide")
            .to_string(),
        "the application claims collide with the member 'sub'"
    );
}
