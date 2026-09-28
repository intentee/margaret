use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::tag::Tag;

use crate::tag_error::TagError;

/// # Errors
///
/// Returns `TagError::MalformedOidcToken` or `TagError::AttributeArguments`.
pub fn read_oidc_token_issuer(args: &AttributeArgs, site: &str) -> Result<Tag, TagError> {
    args.interpret(|reader| {
        reader
            .take_path("issuer")?
            .as_ref()
            .and_then(Tag::from_path)
            .ok_or_else(|| TagError::MalformedOidcToken {
                site: site.to_string(),
            })
    })
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use margaret_attribute_arguments::attribute_args::AttributeArgs;

    use crate::read_oidc_token_issuer::read_oidc_token_issuer;

    fn args(attribute: &Attribute) -> AttributeArgs {
        AttributeArgs::from_attribute(attribute).expect("the arguments parse")
    }

    fn rejection(attribute: &Attribute) -> String {
        read_oidc_token_issuer(&args(attribute), "the site")
            .expect_err("the issuer is rejected")
            .to_string()
    }

    #[test]
    fn reads_the_issuer_tag() {
        let tag = read_oidc_token_issuer(
            &args(&parse_quote!(#[oidc_token(issuer = partner)])),
            "the site",
        )
        .expect("the issuer is read");

        assert_eq!(tag.to_string(), "partner");
    }

    #[test]
    fn rejects_a_token_without_an_issuer() {
        assert_eq!(
            rejection(&parse_quote!(#[oidc_token])),
            "the site must be `issuer = <tag>`"
        );
    }

    #[test]
    fn rejects_an_issuer_that_is_not_a_plain_name() {
        assert_eq!(
            rejection(&parse_quote!(#[oidc_token(issuer = auth::partner)])),
            "the site must be `issuer = <tag>`"
        );
    }

    #[test]
    fn rejects_an_unrecognized_argument() {
        assert_eq!(
            rejection(&parse_quote!(#[oidc_token(issuer = partner, audience = ci)])),
            "failed to read the attribute arguments: attribute 'oidc_token' has an unrecognized argument 'audience'"
        );
    }

    #[test]
    fn rejects_an_issuer_that_is_not_a_path() {
        assert_eq!(
            rejection(&parse_quote!(#[oidc_token(issuer = "partner")])),
            "failed to read the attribute arguments: argument 'issuer' of attribute 'oidc_token' is not a path"
        );
    }
}
