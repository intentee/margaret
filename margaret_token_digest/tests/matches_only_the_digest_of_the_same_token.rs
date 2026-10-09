use margaret_token_digest::token_digest::TokenDigest;

#[test]
fn matches_only_the_digest_of_the_same_token() {
    assert!(TokenDigest::of("presented").matches(&TokenDigest::of("presented")));
    assert!(!TokenDigest::of("presented").matches(&TokenDigest::of("forged")));
}
