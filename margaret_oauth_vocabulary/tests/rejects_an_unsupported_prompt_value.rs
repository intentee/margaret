use margaret_oauth_vocabulary::prompt_value::PromptValue;
use margaret_oauth_vocabulary::prompt_value_parsing::PromptValueParsing;

#[test]
fn rejects_an_unsupported_prompt_value() {
    assert_eq!(
        PromptValue::parse("select_account"),
        PromptValueParsing::Unsupported
    );
}
