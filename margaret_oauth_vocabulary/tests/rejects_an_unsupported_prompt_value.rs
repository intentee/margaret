use margaret_oauth_vocabulary::prompt_value::PromptValue;

#[test]
fn rejects_an_unsupported_prompt_value() {
    assert_eq!(
        "select_account"
            .parse::<PromptValue>()
            .expect_err("account selection is not supported")
            .to_string(),
        "the prompt value 'select_account' is not supported"
    );
}
