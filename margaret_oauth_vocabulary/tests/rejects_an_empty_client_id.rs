use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::client_id_parsing::ClientIdParsing;
use margaret_oauth_vocabulary::client_id_rejection::ClientIdRejection;

#[test]
fn rejects_an_empty_client_id() {
    assert_eq!(
        ClientId::parse(""),
        ClientIdParsing::Rejected(ClientIdRejection::Empty)
    );
}
