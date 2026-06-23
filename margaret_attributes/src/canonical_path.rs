use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FormatResult;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CanonicalPath {
    segments: Vec<String>,
}

impl CanonicalPath {
    pub(crate) fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }

    pub fn segments(&self) -> &[String] {
        &self.segments
    }
}

impl Display for CanonicalPath {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        write!(formatter, "{}", self.segments.join("::"))
    }
}
