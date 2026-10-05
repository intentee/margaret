use margaret_oauth_vocabulary::subject_token_type::SubjectTokenType;

#[test]
fn names_every_subject_token_type() {
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
