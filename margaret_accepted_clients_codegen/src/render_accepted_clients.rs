use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_oauth_vocabulary::grant_type::GrantType;
use margaret_registered_claims::audience::Audience;

use crate::accepted_client_constant::AcceptedClientConstant;
use crate::accepted_client_declaration::AcceptedClientDeclaration;
use crate::accepted_client_item::AcceptedClientItem;
use crate::accepted_clients_module_name::ACCEPTED_CLIENTS_MODULE_NAME;
use crate::clients_module_name::CLIENTS_MODULE_NAME;
use crate::declared_accepted_authentication::DeclaredAcceptedAuthentication;
use crate::declared_accepted_clients::DeclaredAcceptedClients;
use crate::declared_client_credentials::DeclaredClientCredentials;
use crate::declared_code_grant::DeclaredCodeGrant;
use crate::declared_confidential_client::DeclaredConfidentialClient;
use crate::provider_aggregate::ProviderAggregate;

fn grant_type_tokens(grant_type: GrantType) -> TokenStream {
    match grant_type {
        GrantType::AuthorizationCode => quote! {
            margaret::framework::oauth_vocabulary::grant_type::GrantType::AuthorizationCode
        },
        GrantType::ClientCredentials => quote! {
            margaret::framework::oauth_vocabulary::grant_type::GrantType::ClientCredentials
        },
        GrantType::RefreshToken => quote! {
            margaret::framework::oauth_vocabulary::grant_type::GrantType::RefreshToken
        },
        GrantType::TokenExchange => quote! {
            margaret::framework::oauth_vocabulary::grant_type::GrantType::TokenExchange
        },
    }
}

fn client_module_name(client: &AcceptedClientDeclaration) -> String {
    format!(
        "{ACCEPTED_CLIENTS_MODULE_NAME}/{CLIENTS_MODULE_NAME}/{}",
        client.anchor.identifier.field()
    )
}

fn constant_module(
    client: &AcceptedClientDeclaration,
    constant: AcceptedClientConstant,
    value: &TokenStream,
) -> GeneratedModuleTokens {
    let framework_path = constant.framework_path();
    let name = format_ident!("{}", constant.constant_name());

    GeneratedModuleTokens::new(
        format!("{}/{}", client_module_name(client), constant.module_name()),
        quote! { pub const #name: #framework_path = #value; },
    )
}

fn code_grant_tokens(grant: &DeclaredCodeGrant) -> TokenStream {
    match grant {
        DeclaredCodeGrant::Granted(policy) => {
            let consent = policy.consent.tokens();
            let id_token_signing = policy.id_token_signing.tokens();
            let redirect_uris = policy.redirect_uris.iter().map(url::Url::as_str);
            let refresh = if policy.refresh_token {
                quote! { margaret::framework::accepted_clients::refresh_token_grant::RefreshTokenGrant::Granted }
            } else {
                quote! { margaret::framework::accepted_clients::refresh_token_grant::RefreshTokenGrant::Withheld }
            };
            let scopes = policy
                .scopes
                .iter()
                .map(margaret_oauth_vocabulary::scope::Scope::as_str);

            quote! {
                margaret::framework::accepted_clients::authorization_code_grant::AuthorizationCodeGrant::Granted(
                    margaret::framework::accepted_clients::code_grant_policy::CodeGrantPolicy {
                        consent: #consent,
                        id_token_signing: #id_token_signing,
                        redirect_uris: &[#(#redirect_uris),*],
                        refresh: #refresh,
                        scopes: &[#(#scopes),*],
                    }
                )
            }
        }
        DeclaredCodeGrant::Withheld => quote! {
            margaret::framework::accepted_clients::authorization_code_grant::AuthorizationCodeGrant::Withheld
        },
    }
}

fn accepted_client_module(client: &AcceptedClientDeclaration) -> GeneratedModuleTokens {
    let framework_path = AcceptedClientConstant::AcceptedClient.framework_path();
    let authorization_code = code_grant_tokens(&client.authorization_code);
    let client_id = client.client_id.as_str();
    let resources = client.resources.iter().map(Audience::as_str);
    let token_exchange = if client.token_exchange {
        quote! { margaret::framework::accepted_clients::token_exchange_grant::TokenExchangeGrant::Granted }
    } else {
        quote! { margaret::framework::accepted_clients::token_exchange_grant::TokenExchangeGrant::Withheld }
    };

    constant_module(
        client,
        AcceptedClientConstant::AcceptedClient,
        &quote! {
            #framework_path {
                authorization_code: #authorization_code,
                client_id: #client_id,
                resources: &[#(#resources),*],
                token_exchange: #token_exchange,
            }
        },
    )
}

fn confidential_modules(
    client: &AcceptedClientDeclaration,
    confidential: &DeclaredConfidentialClient,
) -> Vec<GeneratedModuleTokens> {
    let privileges = AcceptedClientConstant::ConfidentialPrivileges.framework_path();
    let client_credentials = match &confidential.client_credentials {
        DeclaredClientCredentials::Granted { scopes } => {
            let scopes = scopes
                .iter()
                .map(margaret_oauth_vocabulary::resource_scope::ResourceScope::as_str);

            quote! {
                margaret::framework::accepted_clients::client_credentials_grant::ClientCredentialsGrant::Granted {
                    scopes: &[#(#scopes),*],
                }
            }
        }
        DeclaredClientCredentials::Withheld => quote! {
            margaret::framework::accepted_clients::client_credentials_grant::ClientCredentialsGrant::Withheld
        },
    };
    let introspection = if confidential.introspection {
        quote! { margaret::framework::accepted_clients::introspection_permission::IntrospectionPermission::Permitted }
    } else {
        quote! { margaret::framework::accepted_clients::introspection_permission::IntrospectionPermission::Forbidden }
    };
    let jwks_endpoint_issuer = AcceptedClientConstant::JwksEndpointIssuer.framework_path();
    let client_id = client.client_id.as_str();
    let jwks_uri = confidential.jwks_uri.as_str();

    vec![
        constant_module(
            client,
            AcceptedClientConstant::ConfidentialPrivileges,
            &quote! {
                #privileges {
                    client_credentials: #client_credentials,
                    introspection: #introspection,
                }
            },
        ),
        constant_module(
            client,
            AcceptedClientConstant::JwksEndpointIssuer,
            &quote! { #jwks_endpoint_issuer { issuer: #client_id, jwks_uri: #jwks_uri } },
        ),
    ]
}

fn client_constants(
    authentication: &DeclaredAcceptedAuthentication,
) -> &'static [AcceptedClientConstant] {
    match authentication {
        DeclaredAcceptedAuthentication::PrivateKeyJwt(_) => &[
            AcceptedClientConstant::AcceptedClient,
            AcceptedClientConstant::ConfidentialPrivileges,
            AcceptedClientConstant::JwksEndpointIssuer,
        ],
        DeclaredAcceptedAuthentication::Public => &[AcceptedClientConstant::AcceptedClient],
    }
}

fn client_items(authentication: &DeclaredAcceptedAuthentication) -> &'static [AcceptedClientItem] {
    match authentication {
        DeclaredAcceptedAuthentication::PrivateKeyJwt(_) => &[
            AcceptedClientItem::IssuerKeySet,
            AcceptedClientItem::PolledKeySet,
            AcceptedClientItem::RegisteredClient,
        ],
        DeclaredAcceptedAuthentication::Public => &[AcceptedClientItem::RegisteredClient],
    }
}

fn client_modules(client: &AcceptedClientDeclaration) -> Vec<GeneratedModuleTokens> {
    let submodules = client_constants(&client.authentication)
        .iter()
        .map(|constant| {
            let module = format_ident!("{}", constant.module_name());

            quote! { pub mod #module; }
        });
    let exports = client_items(&client.authentication).iter().map(|item| {
        let framework_path = item.framework_path();

        quote! { pub use #framework_path; }
    });
    let mut modules = vec![
        GeneratedModuleTokens::new(
            client_module_name(client),
            quote! { #(#submodules)* #(#exports)* },
        ),
        accepted_client_module(client),
    ];

    if let DeclaredAcceptedAuthentication::PrivateKeyJwt(confidential) = &client.authentication {
        modules.extend(confidential_modules(client, confidential));
    }

    modules
}

fn aggregate_module(
    aggregate: ProviderAggregate,
    clients: &[AcceptedClientDeclaration],
) -> GeneratedModuleTokens {
    let name = format_ident!("{}", aggregate.constant_name());
    let constant = match aggregate {
        ProviderAggregate::AcceptedResources => {
            let resources = clients
                .iter()
                .flat_map(|client| client.resources.iter().map(Audience::as_str))
                .collect::<BTreeSet<&str>>()
                .into_iter();

            quote! { pub const #name: &[&str] = &[#(#resources),*]; }
        }
        ProviderAggregate::ProviderSupport => {
            let grant_types = clients
                .iter()
                .flat_map(AcceptedClientDeclaration::grant_types)
                .collect::<BTreeSet<GrantType>>()
                .into_iter()
                .map(grant_type_tokens);
            let scopes = clients
                .iter()
                .flat_map(AcceptedClientDeclaration::scopes)
                .collect::<BTreeSet<&str>>()
                .into_iter();

            quote! {
                pub const #name: margaret::framework::oidc_provider::provider_support::ProviderSupport =
                    margaret::framework::oidc_provider::provider_support::ProviderSupport {
                        grant_types: &[#(#grant_types),*],
                        scopes: &[#(#scopes),*],
                    };
            }
        }
    };

    GeneratedModuleTokens::new(
        format!("{ACCEPTED_CLIENTS_MODULE_NAME}/{}", aggregate.module_name()),
        constant,
    )
}

#[must_use]
pub fn render_accepted_clients(
    accepted: &DeclaredAcceptedClients,
    aggregates: &[ProviderAggregate],
) -> Vec<GeneratedModuleTokens> {
    if accepted.clients.is_empty() && aggregates.is_empty() {
        return Vec::new();
    }

    let aggregate_submodules = aggregates.iter().map(|aggregate| {
        let module = format_ident!("{}", aggregate.module_name());

        quote! { pub mod #module; }
    });
    let clients_submodule = (!accepted.clients.is_empty()).then(|| {
        let module = format_ident!("{CLIENTS_MODULE_NAME}");

        quote! { pub mod #module; }
    });
    let mut modules = vec![GeneratedModuleTokens::new(
        ACCEPTED_CLIENTS_MODULE_NAME,
        quote! { #(#aggregate_submodules)* #clients_submodule },
    )];

    modules.extend(
        aggregates
            .iter()
            .map(|aggregate| aggregate_module(*aggregate, &accepted.clients)),
    );

    if !accepted.clients.is_empty() {
        let client_submodules = accepted.clients.iter().map(|client| {
            let module = format_ident!("{}", client.anchor.identifier.field());

            quote! { pub mod #module; }
        });

        modules.push(GeneratedModuleTokens::new(
            format!("{ACCEPTED_CLIENTS_MODULE_NAME}/{CLIENTS_MODULE_NAME}"),
            quote! { #(#client_submodules)* },
        ));
        modules.extend(accepted.clients.iter().flat_map(client_modules));
    }

    modules
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use super::render_accepted_clients;
    use crate::declared_accepted_clients::DeclaredAcceptedClients;
    use crate::provider_aggregate::ProviderAggregate;

    const CLIENTS: &str = "#[issues_tokens(audience = \"session\", issuer = \"https://issuer.example\")]\npub struct Issuer;\n#[accepts_oauth_client(authentication = private_key_jwt(client_credentials(scopes = [\"artifacts:read\"]), introspection, jwks_uri = \"https://portal.example/jwks.json\"), authorization_code(consent = implicit, id_token_signing = rsa, redirect_uris = [\"https://portal.example/callback\"], scopes = [\"openid\", \"profile\"], refresh_token), client_id = \"portal\", resources = [\"artifacts\"], token_exchange)]\npub struct Portal;\n#[accepts_oauth_client(authentication = none, authorization_code(consent = prompted, id_token_signing = elliptic_curve, redirect_uris = [\"http://127.0.0.1:8080/callback\"], scopes = [\"openid\"]), client_id = \"spa\", resources = [\"reports\", \"artifacts\"])]\npub struct Spa;\n";

    fn modules(source: &str, aggregates: &[ProviderAggregate]) -> Vec<GeneratedModuleTokens> {
        let indexed = IndexedSource::new(source);
        let issuance = DeclaredTokenIssuance::read(&indexed.index).expect("the issuance is read");

        render_accepted_clients(
            &DeclaredAcceptedClients::read(&indexed.index, &issuance)
                .expect("the clients are read"),
            aggregates,
        )
    }

    fn named_source(modules: Vec<GeneratedModuleTokens>, name: &str) -> String {
        modules
            .into_iter()
            .find(|module| module.name() == name)
            .expect("the module is generated")
            .format()
            .expect("the module formats")
            .source()
            .to_string()
    }

    fn module_source(name: &str) -> String {
        named_source(modules(CLIENTS, &ProviderAggregate::ALL), name)
    }

    #[test]
    fn renders_nothing_without_accepted_clients_or_referenced_aggregates() {
        assert!(modules("pub struct Plain;\n", &[]).is_empty());
    }

    #[test]
    fn renders_the_referenced_aggregates_of_a_provider_without_clients() {
        let modules = modules("pub struct Plain;\n", &[ProviderAggregate::ProviderSupport]);

        assert_eq!(modules.len(), 2);
        assert_eq!(
            named_source(modules, "accepted_clients"),
            "pub mod provider_support;\n"
        );
    }

    #[test]
    fn omits_the_aggregates_no_endpoint_references() {
        assert_eq!(
            named_source(modules(CLIENTS, &[]), "accepted_clients"),
            "pub mod clients;\n"
        );
    }

    #[test]
    fn declares_the_aggregates_beside_the_clients() {
        assert_eq!(
            module_source("accepted_clients"),
            "pub mod accepted_resources;\npub mod provider_support;\npub mod clients;\n"
        );
    }

    #[test]
    fn declares_a_submodule_per_client_named_after_its_anchor() {
        assert_eq!(
            module_source("accepted_clients/clients"),
            "pub mod portal;\npub mod spa;\n"
        );
    }

    #[test]
    fn re_exports_the_runtime_of_a_confidential_client() {
        assert_eq!(
            module_source("accepted_clients/clients/portal"),
            "pub mod accepted_client;\npub mod confidential_privileges;\npub mod jwks_endpoint_issuer;\npub use margaret::framework::issuer_key_set::issuer_key_set::IssuerKeySet;\npub use margaret::framework::issuer_directory::polled_key_set::PolledKeySet;\npub use margaret::framework::accepted_clients::registered_client::RegisteredClient;\n"
        );
    }

    #[test]
    fn re_exports_the_runtime_of_a_public_client() {
        assert_eq!(
            module_source("accepted_clients/clients/spa"),
            "pub mod accepted_client;\npub use margaret::framework::accepted_clients::registered_client::RegisteredClient;\n"
        );
    }

    #[test]
    fn renders_the_accepted_client_of_a_confidential_client() {
        assert_eq!(
            module_source("accepted_clients/clients/portal/accepted_client"),
            "pub const ACCEPTED_CLIENT: margaret::framework::accepted_clients::accepted_client::AcceptedClient = margaret::framework::accepted_clients::accepted_client::AcceptedClient {\n    authorization_code: margaret::framework::accepted_clients::authorization_code_grant::AuthorizationCodeGrant::Granted(margaret::framework::accepted_clients::code_grant_policy::CodeGrantPolicy {\n        consent: margaret::framework::accepted_clients::consent_policy::ConsentPolicy::Implicit,\n        id_token_signing: margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning::Rsa,\n        redirect_uris: &[\"https://portal.example/callback\"],\n        refresh: margaret::framework::accepted_clients::refresh_token_grant::RefreshTokenGrant::Granted,\n        scopes: &[\"openid\", \"profile\"],\n    }),\n    client_id: \"portal\",\n    resources: &[\"artifacts\"],\n    token_exchange: margaret::framework::accepted_clients::token_exchange_grant::TokenExchangeGrant::Granted,\n};\n"
        );
    }

    #[test]
    fn renders_the_accepted_client_of_a_public_client() {
        assert_eq!(
            module_source("accepted_clients/clients/spa/accepted_client"),
            "pub const ACCEPTED_CLIENT: margaret::framework::accepted_clients::accepted_client::AcceptedClient = margaret::framework::accepted_clients::accepted_client::AcceptedClient {\n    authorization_code: margaret::framework::accepted_clients::authorization_code_grant::AuthorizationCodeGrant::Granted(margaret::framework::accepted_clients::code_grant_policy::CodeGrantPolicy {\n        consent: margaret::framework::accepted_clients::consent_policy::ConsentPolicy::Prompted,\n        id_token_signing: margaret::framework::jwks_secret_store::id_token_signing::IdTokenSigning::EllipticCurve,\n        redirect_uris: &[\"http://127.0.0.1:8080/callback\"],\n        refresh: margaret::framework::accepted_clients::refresh_token_grant::RefreshTokenGrant::Withheld,\n        scopes: &[\"openid\"],\n    }),\n    client_id: \"spa\",\n    resources: &[\"reports\", \"artifacts\"],\n    token_exchange: margaret::framework::accepted_clients::token_exchange_grant::TokenExchangeGrant::Withheld,\n};\n"
        );
    }

    #[test]
    fn renders_the_privileges_of_a_confidential_client() {
        assert_eq!(
            module_source("accepted_clients/clients/portal/confidential_privileges"),
            "pub const CONFIDENTIAL_PRIVILEGES: margaret::framework::accepted_clients::confidential_privileges::ConfidentialPrivileges = margaret::framework::accepted_clients::confidential_privileges::ConfidentialPrivileges {\n    client_credentials: margaret::framework::accepted_clients::client_credentials_grant::ClientCredentialsGrant::Granted {\n        scopes: &[\"artifacts:read\"],\n    },\n    introspection: margaret::framework::accepted_clients::introspection_permission::IntrospectionPermission::Permitted,\n};\n"
        );
    }

    #[test]
    fn renders_the_jwks_endpoint_of_a_confidential_client() {
        assert_eq!(
            module_source("accepted_clients/clients/portal/jwks_endpoint_issuer"),
            "pub const JWKS_ENDPOINT_ISSUER: margaret::framework::issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer = margaret::framework::issuer_directory::jwks_endpoint_issuer::JwksEndpointIssuer {\n    issuer: \"portal\",\n    jwks_uri: \"https://portal.example/jwks.json\",\n};\n"
        );
    }

    #[test]
    fn renders_the_withheld_privileges_of_a_confidential_client() {
        let source = named_source(
            modules(
                "#[issues_tokens(audience = \"session\", issuer = \"https://issuer.example\")]\npub struct Issuer;\n#[accepts_oauth_client(authentication = private_key_jwt(jwks_uri = \"https://portal.example/jwks.json\"), client_id = \"portal\", resources = [\"artifacts\"])]\npub struct Portal;\n",
                &[],
            ),
            "accepted_clients/clients/portal/confidential_privileges",
        );

        assert_eq!(
            source,
            "pub const CONFIDENTIAL_PRIVILEGES: margaret::framework::accepted_clients::confidential_privileges::ConfidentialPrivileges = margaret::framework::accepted_clients::confidential_privileges::ConfidentialPrivileges {\n    client_credentials: margaret::framework::accepted_clients::client_credentials_grant::ClientCredentialsGrant::Withheld,\n    introspection: margaret::framework::accepted_clients::introspection_permission::IntrospectionPermission::Forbidden,\n};\n"
        );
    }

    #[test]
    fn renders_the_union_of_the_accepted_resources() {
        assert_eq!(
            module_source("accepted_clients/accepted_resources"),
            "pub const ACCEPTED_RESOURCES: &[&str] = &[\"artifacts\", \"reports\"];\n"
        );
    }

    #[test]
    fn renders_the_grant_types_and_scopes_the_provider_supports() {
        assert_eq!(
            module_source("accepted_clients/provider_support"),
            "pub const PROVIDER_SUPPORT: margaret::framework::oidc_provider::provider_support::ProviderSupport = margaret::framework::oidc_provider::provider_support::ProviderSupport {\n    grant_types: &[\n        margaret::framework::oauth_vocabulary::grant_type::GrantType::AuthorizationCode,\n        margaret::framework::oauth_vocabulary::grant_type::GrantType::ClientCredentials,\n        margaret::framework::oauth_vocabulary::grant_type::GrantType::RefreshToken,\n        margaret::framework::oauth_vocabulary::grant_type::GrantType::TokenExchange,\n    ],\n    scopes: &[\"artifacts:read\", \"openid\", \"profile\"],\n};\n"
        );
    }
}
