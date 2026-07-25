use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FormatResult;

use proc_macro2::Ident;
use syn::Path;
use syn::PathArguments;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Tag {
    name: Ident,
}

impl Tag {
    #[must_use]
    pub fn from_ident(name: Ident) -> Self {
        Self { name }
    }

    #[must_use]
    pub fn from_path(path: &Path) -> Option<Self> {
        if path.leading_colon.is_some() {
            return None;
        }

        let mut segments = path.segments.iter();
        let (Some(segment), None) = (segments.next(), segments.next()) else {
            return None;
        };

        if !matches!(segment.arguments, PathArguments::None) {
            return None;
        }

        Some(Self {
            name: segment.ident.clone(),
        })
    }
}

impl Display for Tag {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        Display::fmt(&self.name, formatter)
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Ident;
    use proc_macro2::Span;
    use syn::parse_quote;

    use crate::tag::Tag;

    #[test]
    fn reads_a_single_segment_tag() {
        let tag = Tag::from_path(&parse_quote!(jwks)).expect("a single-segment tag");

        assert_eq!(tag.to_string(), "jwks");
    }

    #[test]
    fn builds_a_tag_from_an_identifier() {
        let tag = Tag::from_ident(Ident::new("jwks", Span::call_site()));

        assert_eq!(tag, Tag::from_path(&parse_quote!(jwks)).expect("a tag"));
    }

    #[test]
    fn rejects_a_multi_segment_path() {
        assert!(Tag::from_path(&parse_quote!(endpoints::jwks)).is_none());
    }

    #[test]
    fn rejects_a_leading_colon_path() {
        assert!(Tag::from_path(&parse_quote!(::jwks)).is_none());
    }

    #[test]
    fn rejects_a_generic_path() {
        assert!(Tag::from_path(&parse_quote!(jwks<Token>)).is_none());
    }

    #[test]
    fn tags_with_the_same_name_are_equal() {
        assert_eq!(
            Tag::from_path(&parse_quote!(jwks)).expect("a tag"),
            Tag::from_path(&parse_quote!(jwks)).expect("a tag"),
        );
    }

    #[test]
    fn tags_with_different_names_differ() {
        assert_ne!(
            Tag::from_path(&parse_quote!(jwks)).expect("a tag"),
            Tag::from_path(&parse_quote!(logged)).expect("a tag"),
        );
    }
}
