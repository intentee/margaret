use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::jwks_module_name::JWKS_MODULE_NAME;
use crate::jwks_server_module::JwksServerModule;
use crate::jwks_server_part::JwksServerPart;

#[must_use]
pub fn render_jwks(server: &JwksServerModule) -> GeneratedModuleTokens {
    let roller = server.contains(JwksServerPart::Roller).then(|| {
        quote! {
            pub use margaret::framework::jwks_roller_server::jwks_roller::JwksRoller;
        }
    });
    let handler = server.contains(JwksServerPart::Handler).then(|| {
        quote! {
            pub use margaret::framework::jwks_roller_server::public_jwks_handler::PublicJwksHandler;
        }
    });
    let secret_store = server.contains(JwksServerPart::SecretStore).then(|| {
        quote! {
            pub use margaret::framework::jwks_secret_store::jwks_secret_store::JwksSecretStore;
        }
    });

    GeneratedModuleTokens::new(JWKS_MODULE_NAME, quote! { #roller #handler #secret_store })
}

#[cfg(test)]
mod tests {
    use super::render_jwks;
    use crate::jwks_server_module::JwksServerModule;
    use crate::jwks_server_part::JwksServerPart;

    fn source(server: &JwksServerModule) -> String {
        render_jwks(server)
            .format()
            .expect("the module formats")
            .source()
            .to_string()
    }

    #[test]
    fn re_exports_the_server_side_types_when_present() {
        let mut server = JwksServerModule::default();

        for part in [
            JwksServerPart::Handler,
            JwksServerPart::Roller,
            JwksServerPart::SecretStore,
        ] {
            server.enable_if(part, true);
        }

        let source = source(&server);

        assert!(
            source.contains(
                "pub use margaret::framework::jwks_roller_server::jwks_roller::JwksRoller;"
            )
        );
        assert!(source.contains(
            "pub use margaret::framework::jwks_roller_server::public_jwks_handler::PublicJwksHandler;"
        ));
        assert!(source.contains(
            "pub use margaret::framework::jwks_secret_store::jwks_secret_store::JwksSecretStore;"
        ));
    }

    #[test]
    fn omits_server_side_types_when_absent() {
        let source = source(&JwksServerModule::default());

        assert!(!source.contains("JwksRoller"));
        assert!(!source.contains("PublicJwksHandler"));
        assert!(!source.contains("JwksSecretStore"));
    }
}
