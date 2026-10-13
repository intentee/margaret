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
    pub fn child(&self, segment: &str) -> Self {
        Self {
            segments: self
                .segments
                .iter()
                .cloned()
                .chain([segment.to_string()])
                .collect(),
        }
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

#[cfg(test)]
mod tests {
    use super::CanonicalPath;

    #[test]
    fn names_an_item_nested_under_the_path() {
        assert_eq!(
            CanonicalPath::new(vec!["crate".to_string(), "Metadata".to_string()])
                .child("of_provider")
                .to_string(),
            "crate::Metadata::of_provider"
        );
    }
}
