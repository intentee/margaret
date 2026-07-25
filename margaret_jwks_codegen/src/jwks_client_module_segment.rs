use margaret_attributes::tag::Tag;

#[must_use]
pub fn jwks_client_module_segment(tag: &Tag) -> String {
    tag.to_string()
}

#[cfg(test)]
mod tests {
    use proc_macro2::Ident;
    use proc_macro2::Span;

    use margaret_attributes::tag::Tag;

    use crate::jwks_client_module_segment::jwks_client_module_segment;

    #[test]
    fn uses_the_tag_verbatim_as_the_module_segment() {
        let tag = Tag::from_ident(Ident::new("my_client", Span::call_site()));

        assert_eq!(jwks_client_module_segment(&tag), "my_client");
    }
}
