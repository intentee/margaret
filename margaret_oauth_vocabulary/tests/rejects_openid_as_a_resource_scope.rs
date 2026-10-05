use margaret_oauth_vocabulary::resource_scope::ResourceScope;

#[test]
fn rejects_openid_as_a_resource_scope() {
    assert_eq!(
        "openid"
            .parse::<ResourceScope>()
            .expect_err("openid grants no resource")
            .to_string(),
        "the openid scope asks for an identity, so it cannot grant access to a resource"
    );
}
