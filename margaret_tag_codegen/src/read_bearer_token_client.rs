use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::tag::Tag;

use crate::tag_error::TagError;

/// # Errors
///
/// Returns `TagError::MalformedIntrospectedBearerToken` or `TagError::AttributeArguments`.
pub fn read_bearer_token_client(args: &AttributeArgs, site: &str) -> Result<Tag, TagError> {
    args.interpret(|reader| {
        reader
            .take_path("client")?
            .as_ref()
            .and_then(Tag::from_path)
            .ok_or_else(|| TagError::MalformedIntrospectedBearerToken {
                site: site.to_string(),
            })
    })
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use margaret_attribute_arguments::attribute_args::AttributeArgs;

    use crate::read_bearer_token_client::read_bearer_token_client;

    fn args(attribute: &Attribute) -> AttributeArgs {
        AttributeArgs::from_attribute(attribute).expect("the arguments parse")
    }

    #[test]
    fn reads_the_client_tag() {
        let tag = read_bearer_token_client(
            &args(&parse_quote!(#[bearer_token(client = partner_client)])),
            "the site",
        )
        .expect("the client is read");

        assert_eq!(tag.to_string(), "partner_client");
    }

    #[test]
    fn rejects_a_client_that_is_not_a_path() {
        assert_eq!(
            read_bearer_token_client(
                &args(&parse_quote!(#[bearer_token(client = "partner_client")])),
                "the site"
            )
            .expect_err("the client must be a path")
            .to_string(),
            "failed to read the attribute arguments: argument 'client' of attribute 'bearer_token' is not a path"
        );
    }

    #[test]
    fn rejects_an_introspected_token_without_a_client() {
        assert_eq!(
            read_bearer_token_client(&args(&parse_quote!(#[bearer_token])), "the site")
                .expect_err("the client is required")
                .to_string(),
            "the site must be `client = <tag>`"
        );
    }
}
