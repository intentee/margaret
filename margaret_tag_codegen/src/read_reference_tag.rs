use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::tag::Tag;

use crate::tag_error::TagError;

pub fn read_reference_tag(args: &AttributeArgs, site: &str) -> Result<Tag, TagError> {
    args.reject_unknown_named(&[])?;

    let (Some(path), None) = (args.positional_path(0), args.positional(1)) else {
        return Err(TagError::MalformedReference {
            site: site.to_string(),
        });
    };

    Tag::from_path(path).ok_or_else(|| TagError::MalformedReference {
        site: site.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use margaret_attributes::attribute_args::AttributeArgs;

    use crate::read_reference_tag::read_reference_tag;

    fn args(attribute: Attribute) -> AttributeArgs {
        AttributeArgs::from_attribute(&attribute).expect("the arguments parse")
    }

    #[test]
    fn reads_a_single_plain_tag() {
        let tag = read_reference_tag(&args(parse_quote!(#[endpoint_provider(jwks)])), "the site")
            .expect("the tag is read");

        assert_eq!(tag.to_string(), "jwks");
    }

    #[test]
    fn rejects_a_reference_without_a_tag() {
        assert!(read_reference_tag(&args(parse_quote!(#[endpoint_provider])), "the site").is_err());
    }

    #[test]
    fn rejects_a_reference_with_more_than_one_tag() {
        assert!(
            read_reference_tag(&args(parse_quote!(#[endpoint_provider(a, b)])), "the site")
                .is_err()
        );
    }

    #[test]
    fn rejects_a_reference_that_is_not_a_plain_name() {
        assert!(
            read_reference_tag(&args(parse_quote!(#[endpoint_provider(a::b)])), "the site")
                .is_err()
        );
    }
}
