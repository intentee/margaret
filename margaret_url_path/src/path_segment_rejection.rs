use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PathSegmentRejection {
    #[error("it contains a control character")]
    ControlCharacter,
    #[error("it is a dot segment")]
    DotSegment,
    #[error("it is empty")]
    Empty,
    #[error("it contains a path separator")]
    Separator,
}
