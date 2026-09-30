use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_tag_codegen::trusted_issuer_binding::TrustedIssuerBinding;
use margaret_tag_codegen::trusted_issuer_kind::TrustedIssuerKind;

use crate::trusted_issuers_module_name::TRUSTED_ISSUERS_MODULE_NAME;

fn issuer_exports(kind: TrustedIssuerKind) -> TokenStream {
    match kind {
        TrustedIssuerKind::JwksEndpoint => quote! {
            pub use margaret::framework::trusted_issuer::trusted_issuer::TrustedIssuer;
        },
        TrustedIssuerKind::OidcIssuer => quote! {
            pub use margaret::framework::issuer_metadata::issuer_metadata::IssuerMetadata;
            pub use margaret::framework::trusted_issuer::trusted_issuer::TrustedIssuer;
        },
    }
}

#[must_use]
pub fn render_trusted_issuers(bindings: &[TrustedIssuerBinding]) -> Vec<GeneratedModuleTokens> {
    let submodules = bindings.iter().map(|binding| {
        let module_segment = format_ident!("{}", binding.module_segment);

        quote! { pub mod #module_segment; }
    });
    let mut modules = vec![GeneratedModuleTokens::new(
        TRUSTED_ISSUERS_MODULE_NAME,
        quote! { #(#submodules)* },
    )];

    modules.extend(bindings.iter().map(|binding| {
        GeneratedModuleTokens::new(
            format!("{TRUSTED_ISSUERS_MODULE_NAME}/{}", binding.module_segment),
            issuer_exports(binding.kind),
        )
    }));

    modules
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::tag::Tag;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
    use margaret_tag_codegen::trusted_issuer_binding::TrustedIssuerBinding;
    use margaret_tag_codegen::trusted_issuer_kind::TrustedIssuerKind;
    use syn::Path;

    use super::render_trusted_issuers;

    fn binding(module_segment: &str, kind: TrustedIssuerKind) -> TrustedIssuerBinding {
        let tag_path: Path = syn::parse_str(module_segment).expect("the tag path parses");

        TrustedIssuerBinding {
            declaring: CanonicalPath::new(vec!["crate".to_string(), module_segment.to_string()]),
            kind,
            module_segment: module_segment.to_string(),
            tag: Tag::from_path(&tag_path).expect("the tag is a plain name"),
        }
    }

    fn module_source(modules: Vec<GeneratedModuleTokens>, name: &str) -> String {
        modules
            .into_iter()
            .find(|module| module.name() == name)
            .expect("the module is generated")
            .format()
            .expect("the module formats")
            .source()
            .to_string()
    }

    #[test]
    fn declares_a_submodule_per_trusted_issuer() {
        let root = module_source(
            render_trusted_issuers(&[
                binding("auth", TrustedIssuerKind::JwksEndpoint),
                binding("partner", TrustedIssuerKind::OidcIssuer),
            ]),
            "trusted_issuers",
        );

        assert_eq!(root.trim(), "pub mod auth;\npub mod partner;");
    }

    #[test]
    fn re_exports_the_trusted_issuer_of_a_jwks_endpoint() {
        let submodule = module_source(
            render_trusted_issuers(&[binding("auth", TrustedIssuerKind::JwksEndpoint)]),
            "trusted_issuers/auth",
        );

        assert_eq!(
            submodule.trim(),
            "pub use margaret::framework::trusted_issuer::trusted_issuer::TrustedIssuer;"
        );
    }

    #[test]
    fn re_exports_the_metadata_beside_the_trusted_issuer_of_an_oidc_issuer() {
        let submodule = module_source(
            render_trusted_issuers(&[binding("partner", TrustedIssuerKind::OidcIssuer)]),
            "trusted_issuers/partner",
        );

        assert_eq!(
            submodule.trim(),
            "pub use margaret::framework::issuer_metadata::issuer_metadata::IssuerMetadata;\npub use margaret::framework::trusted_issuer::trusted_issuer::TrustedIssuer;"
        );
    }
}
