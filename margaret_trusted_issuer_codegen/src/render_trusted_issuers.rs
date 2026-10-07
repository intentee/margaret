use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::ext::IdentExt;

use margaret_attributes::tag::Tag;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::declared_trust::DeclaredTrust;
use crate::declared_trusts::DeclaredTrusts;
use crate::trusted_issuer_constant::TrustedIssuerConstant;
use crate::trusted_issuer_group::TrustedIssuerGroup;
use crate::trusted_issuer_item::TrustedIssuerItem;
use crate::trusted_issuers_module_name::TRUSTED_ISSUERS_MODULE_NAME;

fn trust_module_name(tag: &Tag) -> String {
    format!("{TRUSTED_ISSUERS_MODULE_NAME}/{}", tag.ident().unraw())
}

fn trust_module(
    tag: &Tag,
    items: &[TrustedIssuerItem],
    constants: &[TrustedIssuerConstant],
) -> GeneratedModuleTokens {
    let modules = constants.iter().map(|constant| {
        let module = format_ident!("{}", constant.module_name());

        quote! { pub mod #module; }
    });
    let exports = items.iter().map(|item| {
        let framework_path = item.framework_path();

        quote! { pub use #framework_path; }
    });

    GeneratedModuleTokens::new(trust_module_name(tag), quote! { #(#modules)* #(#exports)* })
}

fn constant_module(
    tag: &Tag,
    constant: TrustedIssuerConstant,
    fields: &TokenStream,
) -> GeneratedModuleTokens {
    let framework_path = constant.framework_path();
    let name = format_ident!("{}", constant.constant_name());

    GeneratedModuleTokens::new(
        format!("{}/{}", trust_module_name(tag), constant.module_name()),
        quote! { pub const #name: #framework_path = #framework_path { #fields }; },
    )
}

fn token_trust_module(trust: &DeclaredTrust, issuer: &str) -> GeneratedModuleTokens {
    let audience = trust.audience.as_str();

    constant_module(
        &trust.tag,
        TrustedIssuerConstant::TokenTrust,
        &quote! { audience: #audience, issuer: #issuer, },
    )
}

fn group_modules(group: &TrustedIssuerGroup) -> Vec<GeneratedModuleTokens> {
    let issuer = group.issuer().as_str();
    let mut modules: Vec<GeneratedModuleTokens> = match group {
        TrustedIssuerGroup::Discovered {
            discovery_url,
            lead,
            others,
            ..
        } => {
            let discovery_url = discovery_url.as_str();
            let mut modules = vec![
                trust_module(
                    &lead.tag,
                    &[
                        TrustedIssuerItem::IssuerKeySet,
                        TrustedIssuerItem::IssuerMetadata,
                        TrustedIssuerItem::PolledKeySet,
                        TrustedIssuerItem::TrustedIssuer,
                    ],
                    &[
                        TrustedIssuerConstant::DiscoveredIssuer,
                        TrustedIssuerConstant::TokenTrust,
                    ],
                ),
                constant_module(
                    &lead.tag,
                    TrustedIssuerConstant::DiscoveredIssuer,
                    &quote! { discovery_url: #discovery_url, issuer: #issuer, },
                ),
            ];

            modules.extend(others.iter().map(|other| {
                trust_module(
                    &other.tag,
                    &[TrustedIssuerItem::TrustedIssuer],
                    &[TrustedIssuerConstant::TokenTrust],
                )
            }));

            modules
        }
        TrustedIssuerGroup::JwksEndpoint {
            jwks_uri, trust, ..
        } => {
            let jwks_uri = jwks_uri.as_str();

            vec![
                trust_module(
                    &trust.tag,
                    &[
                        TrustedIssuerItem::IssuerKeySet,
                        TrustedIssuerItem::PolledKeySet,
                        TrustedIssuerItem::TrustedIssuer,
                    ],
                    &[
                        TrustedIssuerConstant::JwksEndpointIssuer,
                        TrustedIssuerConstant::TokenTrust,
                    ],
                ),
                constant_module(
                    &trust.tag,
                    TrustedIssuerConstant::JwksEndpointIssuer,
                    &quote! { issuer: #issuer, jwks_uri: #jwks_uri, },
                ),
            ]
        }
    };

    modules.extend(
        group
            .members()
            .map(|trust| token_trust_module(trust, issuer)),
    );

    modules
}

#[must_use]
pub fn render_trusted_issuers(trusts: &DeclaredTrusts) -> Vec<GeneratedModuleTokens> {
    let mut tags: Vec<&Tag> = trusts
        .bindings()
        .map(|binding| &binding.trust.tag)
        .collect();

    tags.sort_by_key(ToString::to_string);

    let submodules = tags.into_iter().map(|tag| {
        let module = tag.ident();

        quote! { pub mod #module; }
    });
    let mut modules = vec![GeneratedModuleTokens::new(
        TRUSTED_ISSUERS_MODULE_NAME,
        quote! { #(#submodules)* },
    )];

    modules.extend(trusts.groups.iter().flat_map(group_modules));

    modules
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

    use super::render_trusted_issuers;
    use crate::declared_trusts::DeclaredTrusts;

    const SHARED_ISSUER: &str = "#[trusts_oidc_issuer(beta, audience = \"b\", issuer = \"https://issuer.example\")]\npub struct Beta;\n#[trusts_oidc_issuer(alpha, audience = \"a\", issuer = \"https://issuer.example\")]\npub struct Alpha;\n";

    const JWKS_ENDPOINT: &str = "#[provides_jwks_endpoint(ci, audience = \"deploy\", issuer = \"https://ci.example\", jwks_uri = \"https://ci.example/jwks\")]\npub struct Ci;\n";

    fn module_source(lib_source: &str, name: &str) -> String {
        let indexed = IndexedSource::new(lib_source);
        let trusts = DeclaredTrusts::read(&indexed.index).expect("the trusts are read");

        render_trusted_issuers(&trusts)
            .into_iter()
            .find(|module: &GeneratedModuleTokens| module.name() == name)
            .expect("the module is generated")
            .format()
            .expect("the module formats")
            .source()
            .to_string()
    }

    #[test]
    fn declares_a_submodule_per_trust_in_tag_order() {
        assert_eq!(
            module_source(SHARED_ISSUER, "trusted_issuers"),
            "pub mod alpha;\npub mod beta;\n"
        );
    }

    #[test]
    fn exports_the_shared_key_set_and_metadata_from_the_lead_trust_of_a_discovered_issuer() {
        assert_eq!(
            module_source(SHARED_ISSUER, "trusted_issuers/alpha"),
            "pub mod discovered_issuer;\npub mod token_trust;\npub use margaret::framework::issuer_key_set::issuer_key_set::IssuerKeySet;\npub use margaret::framework::issuer_metadata::issuer_metadata::IssuerMetadata;\npub use margaret::framework::issuer_directory::polled_key_set::PolledKeySet;\npub use margaret::framework::trusted_issuer::trusted_issuer::TrustedIssuer;\n"
        );
    }

    #[test]
    fn exports_only_the_trusted_issuer_from_another_trust_of_the_issuer() {
        assert_eq!(
            module_source(SHARED_ISSUER, "trusted_issuers/beta"),
            "pub mod token_trust;\npub use margaret::framework::trusted_issuer::trusted_issuer::TrustedIssuer;\n"
        );
    }

    #[test]
    fn renders_the_discovery_location_of_a_discovered_issuer() {
        assert_eq!(
            module_source(SHARED_ISSUER, "trusted_issuers/alpha/discovered_issuer"),
            "pub const DISCOVERED_ISSUER: margaret::framework::issuer_directory::discovered_issuer::DiscoveredIssuer = margaret::framework::issuer_directory::discovered_issuer::DiscoveredIssuer {\n    discovery_url: \"https://issuer.example/.well-known/openid-configuration\",\n    issuer: \"https://issuer.example\",\n};\n"
        );
    }

    #[test]
    fn renders_the_token_trust_of_each_trust() {
        assert_eq!(
            module_source(SHARED_ISSUER, "trusted_issuers/beta/token_trust"),
            "pub const TOKEN_TRUST: margaret::framework::token_trust::token_trust::TokenTrust = margaret::framework::token_trust::token_trust::TokenTrust {\n    audience: \"b\",\n    issuer: \"https://issuer.example\",\n};\n"
        );
    }

    #[test]
    fn exports_the_key_set_of_a_jwks_endpoint_issuer_without_metadata() {
        assert_eq!(
            module_source(JWKS_ENDPOINT, "trusted_issuers/ci"),
            "pub mod jwks_endpoint_issuer;\npub mod token_trust;\npub use margaret::framework::issuer_key_set::issuer_key_set::IssuerKeySet;\npub use margaret::framework::issuer_directory::polled_key_set::PolledKeySet;\npub use margaret::framework::trusted_issuer::trusted_issuer::TrustedIssuer;\n"
        );
    }

    #[test]
    fn renders_the_jwks_uri_of_a_jwks_endpoint_issuer() {
        assert_eq!(
            module_source(JWKS_ENDPOINT, "trusted_issuers/ci/jwks_endpoint_issuer"),
            "pub const JWKS_ENDPOINT_ISSUER: margaret::framework::issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer = margaret::framework::issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer {\n    issuer: \"https://ci.example\",\n    jwks_uri: \"https://ci.example/jwks\",\n};\n"
        );
    }

    #[test]
    fn names_the_module_of_a_raw_identifier_tag_without_its_prefix() {
        assert_eq!(
            module_source(
                "#[trusts_oidc_issuer(r#async, audience = \"a\", issuer = \"https://issuer.example\")]\npub struct Async;\n",
                "trusted_issuers"
            ),
            "pub mod r#async;\n"
        );
    }
}
