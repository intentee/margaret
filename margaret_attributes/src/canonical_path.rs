use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FormatResult;

use heck::ToSnakeCase;

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

    pub fn field_name(&self) -> String {
        self.segments
            .last()
            .expect("a canonical path has at least one segment")
            .to_snake_case()
    }
}

impl Display for CanonicalPath {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        write!(formatter, "{}", self.segments.join("::"))
    }
}

#[cfg(test)]
mod tests {
    use super::CanonicalPath;

    #[test]
    fn field_name_is_the_snake_cased_leaf() {
        let path = CanonicalPath::new(vec![
            "crate".to_string(),
            "routes".to_string(),
            "GetGreeting".to_string(),
        ]);

        assert_eq!(path.field_name(), "get_greeting");
    }
}
