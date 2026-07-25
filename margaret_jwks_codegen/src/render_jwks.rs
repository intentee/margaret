use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::jwks_module_name::JWKS_MODULE_NAME;

#[must_use]
pub fn render_jwks(
    has_roller: bool,
    has_handler: bool,
    has_client: bool,
    has_verifier: bool,
) -> GeneratedModuleTokens {
    let roller = has_roller.then(|| {
        quote! {
            pub use margaret_jwks_roller_server::jwks_roller::JwksRoller;
        }
    });
    let handler = has_handler.then(|| {
        quote! {
            pub use margaret_jwks_roller_server::public_jwks_handler::PublicJwksHandler;
        }
    });
    let client = has_client.then(|| {
        quote! {
            pub use margaret_jwks_client::jwks_client::JwksClient;
        }
    });
    let verifier = has_verifier.then(|| {
        quote! {
            pub use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;
        }
    });

    GeneratedModuleTokens::new(
        JWKS_MODULE_NAME,
        quote! { #roller #handler #client #verifier },
    )
}

#[cfg(test)]
mod tests {
    use super::render_jwks;

    fn source(has_roller: bool, has_handler: bool, has_client: bool, has_verifier: bool) -> String {
        render_jwks(has_roller, has_handler, has_client, has_verifier)
            .format()
            .expect("the jwks module formats")
            .source()
            .to_string()
    }

    #[test]
    fn re_exports_the_roller_and_handler_when_the_roller_is_used() {
        let source = source(true, true, false, false);

        assert!(source.contains("pub use margaret_jwks_roller_server::jwks_roller::JwksRoller;"));
        assert!(source.contains(
            "pub use margaret_jwks_roller_server::public_jwks_handler::PublicJwksHandler;"
        ));
        assert!(!source.contains("JwksClient"));
        assert!(!source.contains("PublicJwksVerifier"));
    }

    #[test]
    fn re_exports_the_client_without_the_verifier_when_only_the_endpoint_is_provided() {
        let source = source(false, false, true, false);

        assert!(source.contains("pub use margaret_jwks_client::jwks_client::JwksClient;"));
        assert!(!source.contains("PublicJwksVerifier"));
        assert!(!source.contains("JwksRoller"));
    }

    #[test]
    fn re_exports_the_verifier_when_it_is_injected() {
        let source = source(false, false, true, true);

        assert!(source.contains("pub use margaret_jwks_client::jwks_client::JwksClient;"));
        assert!(source.contains(
            "pub use margaret_jwks_client::public_jwks_verifier::PublicJwksVerifier;"
        ));
    }
}
