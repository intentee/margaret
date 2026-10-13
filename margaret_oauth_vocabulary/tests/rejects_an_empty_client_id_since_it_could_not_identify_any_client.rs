use margaret_oauth_vocabulary::client_id::ClientId;
use margaret_oauth_vocabulary::client_id_parsing::ClientIdParsing;
use margaret_oauth_vocabulary::client_id_rejection::ClientIdRejection;

#[test]
fn rejects_an_empty_client_id_since_it_could_not_identify_any_client() {
    assert_eq!(
        ClientId::parse(""),
        ClientIdParsing::Rejected(ClientIdRejection::Empty)
    );
}
