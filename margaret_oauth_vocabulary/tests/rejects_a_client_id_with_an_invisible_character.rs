use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::client_id_parsing::ClientIdParsing;
use margaret_oauth_vocabulary::client_id_rejection::ClientIdRejection;

#[test]
fn rejects_a_client_id_with_an_invisible_character() {
    assert_eq!(
        ClientId::parse("client\u{7f}"),
        ClientIdParsing::Rejected(ClientIdRejection::InvisibleCharacter)
    );
}
