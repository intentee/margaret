use margaret_accepted_clients::client_refusal::ClientRefusal;

#[test]
fn describes_every_client_refusal() {
    assert_eq!(
        [
            ClientRefusal::ConflictingClientIds,
            ClientRefusal::MalformedCredentials,
            ClientRefusal::MissingCredentials,
            ClientRefusal::SecretRequired,
            ClientRefusal::UnexpectedSecret,
            ClientRefusal::UnknownClient,
            ClientRefusal::WrongSecret,
        ]
        .map(|refusal| refusal.to_string()),
        [
            "the basic credentials and the form name different clients",
            "the client credentials are malformed",
            "the request carries no client credentials",
            "the confidential client presented no secret",
            "the public client presented a secret",
            "the client is not accepted",
            "the client secret does not match",
        ]
    );
}
