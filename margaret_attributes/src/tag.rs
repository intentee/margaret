use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use proc_macro2::Ident;
use syn::Path;

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
        path.get_ident().map(|name| Self { name: name.clone() })
    }

    #[must_use]
    pub fn ident(&self) -> &Ident {
        &self.name
    }
}

impl Display for Tag {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
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
    fn reads_a_single_segment_tag_so_a_bare_identifier_can_name_a_tag() {
        let tag = Tag::from_path(&parse_quote!(jwks)).expect("a single-segment tag");

        assert_eq!(tag.to_string(), "jwks");
    }

    #[test]
    fn builds_a_tag_from_an_identifier_so_it_matches_the_tag_read_from_a_path() {
        let tag = Tag::from_ident(Ident::new("jwks", Span::call_site()));

        assert_eq!(tag, Tag::from_path(&parse_quote!(jwks)).expect("a tag"));
    }

    #[test]
    fn rejects_a_multi_segment_path_because_a_tag_is_a_bare_identifier() {
        assert!(Tag::from_path(&parse_quote!(endpoints::jwks)).is_none());
    }

    #[test]
    fn rejects_a_leading_colon_path_because_a_tag_is_a_bare_identifier() {
        assert!(Tag::from_path(&parse_quote!(::jwks)).is_none());
    }

    #[test]
    fn rejects_a_path_with_generic_arguments_because_a_tag_is_a_bare_identifier() {
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
    fn tags_with_different_names_differ_so_each_tag_identifies_its_own_declaration() {
        assert_ne!(
            Tag::from_path(&parse_quote!(jwks)).expect("a tag"),
            Tag::from_path(&parse_quote!(logged)).expect("a tag"),
        );
    }
}
