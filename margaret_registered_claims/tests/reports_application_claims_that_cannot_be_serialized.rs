use std::collections::BTreeMap;

use serde_json::Map;

use margaret_registered_claims::merge_claims::merge_claims;

#[test]
fn reports_application_claims_that_cannot_be_serialized() {
    assert_eq!(
        merge_claims(Map::new(), &BTreeMap::from([((1, 2), "demo")]))
            .expect_err("the claims cannot be serialized")
            .to_string(),
        "the application claims could not be serialized to json: key must be a string"
    );
}
