use margaret_attribute_arguments::attribute_args::AttributeArgs;
use margaret_attributes::tag::Tag;

use crate::bearer_token_addressee::BearerTokenAddressee;
use crate::tag_error::TagError;

/// # Errors
///
/// Returns `TagError::MalformedBearerToken` unless exactly one of `client`, `issuer` or
/// `resource` names a plain tag, and `TagError::AttributeArguments` for malformed arguments.
pub fn read_bearer_token_addressee(
    args: &AttributeArgs,
    site: &str,
) -> Result<BearerTokenAddressee, TagError> {
    args.interpret(|reader| {
        let malformed = || TagError::MalformedBearerToken {
            site: site.to_string(),
        };
        let mut addressees = Vec::new();

        if let Some(client) = reader.take_path("client")? {
            addressees.push(
                Tag::from_path(&client)
                    .map(BearerTokenAddressee::Client)
                    .ok_or_else(malformed)?,
            );
        }

        if let Some(issuer) = reader.take_path("issuer")? {
            addressees.push(
                Tag::from_path(&issuer)
                    .map(BearerTokenAddressee::Issuer)
                    .ok_or_else(malformed)?,
            );
        }

        if let Some(resource) = reader.take_path("resource")? {
            addressees.push(
                Tag::from_path(&resource)
                    .map(BearerTokenAddressee::Resource)
                    .ok_or_else(malformed)?,
            );
        }

        let mut addressees = addressees.into_iter();

        match addressees.next() {
            Some(addressee) if addressees.next().is_none() => Ok(addressee),
            Some(_) | None => Err(malformed()),
        }
    })
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use margaret_attribute_arguments::attribute_args::AttributeArgs;

    use super::read_bearer_token_addressee;
    use crate::bearer_token_addressee::BearerTokenAddressee;

    fn read(attribute: &Attribute) -> Result<BearerTokenAddressee, String> {
        read_bearer_token_addressee(
            &AttributeArgs::from_attribute(attribute).expect("the arguments parse"),
            "the site",
        )
        .map_err(|error| error.to_string())
    }

    #[test]
    fn reads_the_client_that_introspects_the_token() {
        assert!(matches!(
            read(&parse_quote!(#[bearer_token(client = blog)])),
            Ok(BearerTokenAddressee::Client(tag)) if tag.to_string() == "blog"
        ));
    }

    #[test]
    fn reads_the_issuer_that_signs_the_token() {
        assert!(matches!(
            read(&parse_quote!(#[bearer_token(issuer = partner)])),
            Ok(BearerTokenAddressee::Issuer(tag)) if tag.to_string() == "partner"
        ));
    }

    #[test]
    fn reads_the_resource_the_token_addresses() {
        assert!(matches!(
            read(&parse_quote!(#[bearer_token(resource = attachments)])),
            Ok(BearerTokenAddressee::Resource(tag)) if tag.to_string() == "attachments"
        ));
    }

    #[test]
    fn rejects_a_token_without_an_addressee() {
        assert!(matches!(
            read(&parse_quote!(#[bearer_token])),
            Err(message) if message == "the site must be exactly one of `client = <tag>`, `issuer = <tag>` or `resource = <tag>`"
        ));
    }

    #[test]
    fn rejects_a_token_with_two_addressees() {
        assert!(matches!(
            read(&parse_quote!(#[bearer_token(issuer = partner, resource = attachments)])),
            Err(message) if message == "the site must be exactly one of `client = <tag>`, `issuer = <tag>` or `resource = <tag>`"
        ));
    }

    #[test]
    fn rejects_addressees_that_are_not_plain_names() {
        assert!(
            [
                read(&parse_quote!(#[bearer_token(client = clients::blog)])),
                read(&parse_quote!(#[bearer_token(issuer = auth::partner)])),
                read(&parse_quote!(#[bearer_token(resource = files::attachments)])),
            ]
            .iter()
            .all(Result::is_err)
        );
    }

    #[test]
    fn rejects_an_unrecognized_argument() {
        assert!(matches!(
            read(&parse_quote!(#[bearer_token(issuer = partner, audience = ci)])),
            Err(message) if message == "failed to read the attribute arguments: attribute 'bearer_token' has an unrecognized argument 'audience'"
        ));
    }

    #[test]
    fn rejects_addressees_that_are_not_paths() {
        assert_eq!(
            [
                read(&parse_quote!(#[bearer_token(client = "blog")])),
                read(&parse_quote!(#[bearer_token(issuer = "partner")])),
                read(&parse_quote!(#[bearer_token(resource = "attachments")])),
            ]
            .map(Result::err),
            [
                Some("failed to read the attribute arguments: argument 'client' of attribute 'bearer_token' is not a path".to_string()),
                Some("failed to read the attribute arguments: argument 'issuer' of attribute 'bearer_token' is not a path".to_string()),
                Some("failed to read the attribute arguments: argument 'resource' of attribute 'bearer_token' is not a path".to_string()),
            ]
        );
    }
}
