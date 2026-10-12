use margaret_token_digest::random_token::random_token;

#[test]
fn generates_distinct_url_safe_random_tokens_so_they_fit_urls_and_resist_guessing() {
    let first = random_token();

    assert_eq!(first.len(), 43);
    assert!(
        first
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    );
    assert_ne!(first, random_token());
}
