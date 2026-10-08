use crate::prompt_value::PromptValue;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptValueParsing {
    Accepted(PromptValue),
    Unsupported,
}
