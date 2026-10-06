use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_oidc_provider_codegen::oidc_provider_item::OidcProviderItem;
use margaret_oidc_provider_codegen::oidc_provider_item_path::oidc_provider_item_path;
use margaret_oidc_provider_codegen::provider_endpoint_paths_path::provider_endpoint_paths_path;
use margaret_oidc_provider_codegen::subject_token_exchanger_path::subject_token_exchanger_path;
use margaret_tag_codegen::subject_token_exchanger_binding::SubjectTokenExchangerBinding;
use margaret_trusted_issuer_codegen::trusted_issuer_canonical_path::trusted_issuer_canonical_path;

use crate::provider_state_canonical_path::provider_state_canonical_path;
use crate::server_secret_store_canonical_path::server_secret_store_canonical_path;

fn framework_path(segments: &[&str]) -> CanonicalPath {
    CanonicalPath::new(
        ["margaret", "framework"]
            .iter()
            .chain(segments)
            .map(|segment| (*segment).to_string())
            .collect(),
    )
}

fn constructed(
    dependencies: Vec<FrameworkDependency>,
    outcome: ConstructorOutcome,
    enablement: FrameworkEnablement,
    provided: CanonicalPath,
) -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies,
            is_async: false,
            method: "create".to_string(),
            outcome,
        },
        enablement,
        injection: FrameworkInjectionRole::Unmarked,
        provided,
    }
}

fn item(provided: OidcProviderItem) -> FrameworkDependency {
    FrameworkDependency::Provider(oidc_provider_item_path(provided))
}

fn borrowed_item(provided: OidcProviderItem) -> FrameworkDependency {
    FrameworkDependency::BorrowedProvider(oidc_provider_item_path(provided))
}

fn endpoint(
    dependencies: Vec<FrameworkDependency>,
    provided: OidcProviderItem,
) -> FrameworkProvider {
    constructed(
        dependencies,
        ConstructorOutcome::Infallible,
        FrameworkEnablement::WhenReferenced,
        oidc_provider_item_path(provided),
    )
}

fn provider_state_storage() -> FrameworkProvider {
    FrameworkProvider {
        construction: FrameworkConstruction::UriSelected {
            argument_name: "oidc-provider-state-storage".to_string(),
            resolver: framework_path(&[
                "provider_state_storage_selection",
                "resolve_provider_state_storage",
                "resolve_provider_state_storage",
            ]),
            value_type: framework_path(&[
                "provider_state_storage_selection",
                "provider_state_storage_uri",
                "ProviderStateStorageUri",
            ]),
        },
        enablement: FrameworkEnablement::Dependency,
        injection: FrameworkInjectionRole::Unmarked,
        provided: provider_state_canonical_path(),
    }
}

fn subject_token_exchanger(
    SubjectTokenExchangerBinding { declaring, issuer }: &SubjectTokenExchangerBinding,
) -> FrameworkProvider {
    constructed(
        vec![
            FrameworkDependency::Provider(trusted_issuer_canonical_path(&issuer.module_segment)),
            FrameworkDependency::SingletonView(declaring.clone()),
        ],
        ConstructorOutcome::Infallible,
        FrameworkEnablement::Dependency,
        subject_token_exchanger_path(&issuer.module_segment),
    )
}

pub(crate) fn oidc_provider_framework_providers(
    accepted_clients: &[CanonicalPath],
    exchangers: &[SubjectTokenExchangerBinding],
) -> Vec<FrameworkProvider> {
    let state = || FrameworkDependency::Provider(provider_state_canonical_path());
    let secret_store = || FrameworkDependency::Provider(server_secret_store_canonical_path());
    let mut providers = vec![
        provider_state_storage(),
        constructed(
            vec![
                FrameworkDependency::SingletonViews {
                    singletons: accepted_clients.to_vec(),
                    view: framework_path(&[
                        "accepted_clients",
                        "declares_accepted_client",
                        "DeclaresAcceptedClient",
                    ]),
                },
                FrameworkDependency::BorrowedTokenIssuance,
            ],
            ConstructorOutcome::Fallible,
            FrameworkEnablement::Dependency,
            oidc_provider_item_path(OidcProviderItem::AcceptedClients),
        ),
        constructed(
            vec![FrameworkDependency::Providers(
                exchangers
                    .iter()
                    .map(|binding| subject_token_exchanger_path(&binding.issuer.module_segment))
                    .collect(),
            )],
            ConstructorOutcome::Infallible,
            FrameworkEnablement::Dependency,
            oidc_provider_item_path(OidcProviderItem::SubjectTokenExchangers),
        ),
        constructed(
            vec![
                FrameworkDependency::BorrowedTokenIssuance,
                FrameworkDependency::Constant(provider_endpoint_paths_path()),
            ],
            ConstructorOutcome::Fallible,
            FrameworkEnablement::Dependency,
            oidc_provider_item_path(OidcProviderItem::ProviderEndpoints),
        ),
        endpoint(
            vec![
                item(OidcProviderItem::AcceptedClients),
                state(),
                item(OidcProviderItem::ProviderEndpoints),
                FrameworkDependency::TokenIssuance,
            ],
            OidcProviderItem::AuthorizationEndpoint,
        ),
        endpoint(
            vec![state(), FrameworkDependency::TokenIssuance],
            OidcProviderItem::ConsentEndpoint,
        ),
        endpoint(
            vec![item(OidcProviderItem::AcceptedClients), secret_store()],
            OidcProviderItem::IntrospectionEndpoint,
        ),
        constructed(
            vec![
                borrowed_item(OidcProviderItem::AcceptedClients),
                borrowed_item(OidcProviderItem::ProviderEndpoints),
                FrameworkDependency::BorrowedTokenIssuance,
            ],
            ConstructorOutcome::Fallible,
            FrameworkEnablement::WhenReferenced,
            oidc_provider_item_path(OidcProviderItem::ProviderMetadataHandler),
        ),
        endpoint(
            vec![
                item(OidcProviderItem::AcceptedClients),
                secret_store(),
                state(),
            ],
            OidcProviderItem::RevocationEndpoint,
        ),
        endpoint(
            vec![
                item(OidcProviderItem::AcceptedClients),
                state(),
                item(OidcProviderItem::SubjectTokenExchangers),
                secret_store(),
                FrameworkDependency::TokenIssuance,
            ],
            OidcProviderItem::TokenEndpoint,
        ),
        endpoint(
            vec![secret_store(), FrameworkDependency::BorrowedTokenIssuance],
            OidcProviderItem::UserinfoEndpoint,
        ),
    ];

    providers.extend(exchangers.iter().map(subject_token_exchanger));

    providers
}
