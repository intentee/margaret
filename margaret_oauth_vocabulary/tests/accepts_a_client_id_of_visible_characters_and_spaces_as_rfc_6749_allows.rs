use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::client_id_parsing::ClientIdParsing;

#[test]
fn accepts_a_client_id_of_visible_characters_and_spaces_as_rfc_6749_allows() {
    assert!(matches!(
        ClientId::parse("s6BhdRkqt3 ~"),
        ClientIdParsing::Accepted(client_id) if client_id.to_string() == "s6BhdRkqt3 ~"
    ));
}
