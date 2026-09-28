use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CanonicalPath {
    segments: Vec<String>,
}

impl CanonicalPath {
    #[must_use]
    pub fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }

    #[must_use]
    pub fn segments(&self) -> &[String] {
        &self.segments
    }
}

impl Display for CanonicalPath {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(formatter, "{}", self.segments.join("::"))
    }
}
