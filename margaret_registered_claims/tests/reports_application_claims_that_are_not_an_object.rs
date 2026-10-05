use serde_json::Map;

use margaret_registered_claims::merge_claims::merge_claims;

#[test]
fn reports_application_claims_that_are_not_an_object() {
    assert_eq!(
        merge_claims(Map::new(), &["demo"])
            .expect_err("the claims are not an object")
            .to_string(),
        "the application claims are not a json object"
    );
}
