use crate::subject_token_type::SubjectTokenType;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubjectTokenTypeParsing {
    Accepted(SubjectTokenType),
    Unsupported,
}
