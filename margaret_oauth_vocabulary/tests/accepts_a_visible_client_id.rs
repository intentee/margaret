use margaret_oauth_vocabulary::client_id::ClientId;

#[test]
fn accepts_a_visible_client_id() {
    assert_eq!(
        "s6BhdRkqt3 ~"
            .parse::<ClientId>()
            .expect("the client identifier is visible")
            .to_string(),
        "s6BhdRkqt3 ~"
    );
}
