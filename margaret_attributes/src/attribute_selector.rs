use syn::Ident;
use syn::Path;

use crate::attribute_error::AttributeError;
use crate::format_path::format_path;

pub struct AttributeSelector {
    path: Path,
}

impl AttributeSelector {
    pub fn parse(input: &str) -> Result<Self, AttributeError> {
        match syn::parse_str::<Path>(input) {
            Ok(path) => Ok(Self { path }),
            Err(source) => Err(AttributeError::InvalidSelector {
                input: input.to_string(),
                source,
            }),
        }
    }

    pub fn from_path(path: Path) -> Self {
        Self { path }
    }

    pub fn matches(&self, attribute_path: &Path) -> bool {
        let selector_length = self.path.segments.len();
        let attribute_length = attribute_path.segments.len();

        if selector_length > attribute_length {
            return false;
        }

        let offset = attribute_length - selector_length;

        self.path
            .segments
            .iter()
            .zip(attribute_path.segments.iter().skip(offset))
            .all(|(selector_segment, attribute_segment)| {
                selector_segment.ident == attribute_segment.ident
            })
    }

    pub(crate) fn display_path(&self) -> String {
        format_path(&self.path)
    }

    pub(crate) fn leaf_ident(&self) -> &Ident {
        &self
            .path
            .segments
            .last()
            .expect("a selector path has at least one segment")
            .ident
    }
}

#[cfg(test)]
mod tests {
    use syn::Path;
    use syn::parse_quote;

    use crate::attribute_selector::AttributeSelector;

    fn selector(input: &str) -> AttributeSelector {
        AttributeSelector::parse(input).expect("the selector parses")
    }

    #[test]
    fn matches_a_qualified_attribute_by_its_leaf_segment() {
        let attribute_path: Path = parse_quote!(ns::tagged);

        assert!(selector("tagged").matches(&attribute_path));
    }

    #[test]
    fn matches_a_qualified_attribute_by_its_full_path() {
        let attribute_path: Path = parse_quote!(ns::tagged);

        assert!(selector("ns::tagged").matches(&attribute_path));
    }

    #[test]
    fn a_selector_longer_than_the_attribute_path_matches_nothing() {
        let attribute_path: Path = parse_quote!(singleton);

        assert!(!selector("deeply::nested::singleton").matches(&attribute_path));
    }

    #[test]
    fn a_selector_with_a_different_segment_matches_nothing() {
        let attribute_path: Path = parse_quote!(singleton);

        assert!(!selector("does_not_exist").matches(&attribute_path));
    }

    #[test]
    fn an_invalid_selector_is_rejected() {
        let message = AttributeSelector::parse("123")
            .err()
            .expect("a non-path selector is rejected")
            .to_string();

        assert!(message.contains("invalid attribute selector"));
    }

    #[test]
    fn display_path_renders_the_written_selector() {
        assert_eq!(selector("ns::tagged").display_path(), "ns::tagged");
    }

    #[test]
    fn from_path_builds_a_matching_selector() {
        let selector = AttributeSelector::from_path(parse_quote!(intercepts));

        assert!(selector.matches(&parse_quote!(crate::markers::intercepts)));
    }
}
