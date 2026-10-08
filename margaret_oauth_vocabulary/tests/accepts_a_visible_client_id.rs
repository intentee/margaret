use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::client_id_parsing::ClientIdParsing;

#[test]
fn accepts_a_visible_client_id() {
    assert!(matches!(
        ClientId::parse("s6BhdRkqt3 ~"),
        ClientIdParsing::Accepted(client_id) if client_id.to_string() == "s6BhdRkqt3 ~"
    ));
}
