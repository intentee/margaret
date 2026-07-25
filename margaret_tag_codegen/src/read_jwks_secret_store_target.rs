use margaret_attributes::attribute_args::AttributeArgs;
use margaret_attributes::tag::Tag;

use crate::jwks_secret_store_target::JwksSecretStoreTarget;
use crate::tag_error::TagError;

pub fn read_jwks_secret_store_target(
    args: &AttributeArgs,
    site: &str,
) -> Result<JwksSecretStoreTarget, TagError> {
    args.interpret(|reader| {
        let server = reader.take_flag("server");
        let client = reader.take_path("client")?;

        match (server, client) {
            (true, None) => Ok(JwksSecretStoreTarget::Server),
            (false, Some(path)) => Tag::from_path(&path)
                .map(JwksSecretStoreTarget::Client)
                .ok_or_else(|| TagError::MalformedJwksSecretStore {
                    site: site.to_string(),
                }),
            _ => Err(TagError::MalformedJwksSecretStore {
                site: site.to_string(),
            }),
        }
    })
}

#[cfg(test)]
mod tests {
    use syn::Attribute;
    use syn::parse_quote;

    use margaret_attributes::attribute_args::AttributeArgs;

    use crate::jwks_secret_store_target::JwksSecretStoreTarget;
    use crate::read_jwks_secret_store_target::read_jwks_secret_store_target;

    fn args(attribute: Attribute) -> AttributeArgs {
        AttributeArgs::from_attribute(&attribute).expect("the arguments parse")
    }

    #[test]
    fn reads_the_server_target() {
        let target =
            read_jwks_secret_store_target(&args(parse_quote!(#[jwks_secret_store(server)])), "site")
                .expect("the server target is read");

        assert_eq!(target, JwksSecretStoreTarget::Server);
    }

    #[test]
    fn reads_a_client_target_tag() {
        let target = read_jwks_secret_store_target(
            &args(parse_quote!(#[jwks_secret_store(client = auth)])),
            "site",
        )
        .expect("the client target is read");

        assert!(matches!(target, JwksSecretStoreTarget::Client(tag) if tag.to_string() == "auth"));
    }

    #[test]
    fn rejects_an_empty_target() {
        assert!(
            read_jwks_secret_store_target(&args(parse_quote!(#[jwks_secret_store])), "site")
                .is_err()
        );
    }

    #[test]
    fn rejects_both_the_server_and_a_client() {
        assert!(
            read_jwks_secret_store_target(
                &args(parse_quote!(#[jwks_secret_store(server, client = auth)])),
                "site",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_a_client_target_that_is_not_a_plain_name() {
        assert!(
            read_jwks_secret_store_target(
                &args(parse_quote!(#[jwks_secret_store(client = auth::inner)])),
                "site",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_a_client_target_that_is_not_a_path() {
        assert!(
            read_jwks_secret_store_target(
                &args(parse_quote!(#[jwks_secret_store(client = "auth")])),
                "site",
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_an_unrecognized_argument() {
        assert!(
            read_jwks_secret_store_target(
                &args(parse_quote!(#[jwks_secret_store(server, extra)])),
                "site",
            )
            .is_err()
        );
    }
}
