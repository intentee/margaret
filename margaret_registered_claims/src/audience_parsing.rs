use crate::audience::Audience;

#[derive(Debug, Eq, PartialEq)]
pub enum AudienceParsing {
    Accepted(Audience),
    Empty,
}
