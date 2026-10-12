use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;

#[test]
fn names_every_subject_token_type_by_its_rfc_8693_urn_so_servers_recognize_it() {
    assert_eq!(
        [
            SubjectTokenType::AccessToken,
            SubjectTokenType::IdToken,
            SubjectTokenType::Jwt,
        ]
        .map(SubjectTokenType::urn),
        [
            "urn:ietf:params:oauth:token-type:access_token",
            "urn:ietf:params:oauth:token-type:id_token",
            "urn:ietf:params:oauth:token-type:jwt",
        ]
    );
}
