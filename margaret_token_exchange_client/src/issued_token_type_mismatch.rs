#[derive(Debug, Eq, PartialEq)]
pub enum IssuedTokenTypeMismatch {
    Missing,
    Other { issued_token_type: String },
}
