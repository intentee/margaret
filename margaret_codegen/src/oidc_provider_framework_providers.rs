use margaret_accepted_clients_codegen::provider_aggregate::ProviderAggregate;
use margaret_accepted_clients_codegen::provider_aggregate_path::provider_aggregate_path;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_environment_variable_codegen::environment_variable_name::EnvironmentVariableName;
use margaret_environment_variable_codegen::framework_environment_variable::FrameworkEnvironmentVariable;
use margaret_oidc_provider_codegen::oidc_provider_item::OidcProviderItem;
use margaret_oidc_provider_codegen::oidc_provider_item_path::oidc_provider_item_path;
use margaret_oidc_provider_codegen::provider_endpoints_path::provider_endpoints_path;
use margaret_oidc_provider_codegen::subject_token_exchanger_path::subject_token_exchanger_path;
use margaret_tag_codegen::subject_token_exchanger_binding::SubjectTokenExchangerBinding;
use margaret_trusted_issuer_codegen::trusted_issuer_item::TrustedIssuerItem;
use margaret_trusted_issuer_codegen::trusted_issuer_item_path::trusted_issuer_item_path;

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
        construction: FrameworkConstruction::Resolved {
            dependencies: vec![FrameworkDependency::EnvironmentVariable {
                name: EnvironmentVariableName::from(
                    FrameworkEnvironmentVariable::OidcProviderStateStorage,
                ),
                value_type: framework_path(&[
                    "provider_state_storage_selection",
                    "provider_state_storage_uri",
                    "ProviderStateStorageUri",
                ]),
            }],
            resolver: framework_path(&[
                "provider_state_storage_selection",
                "resolve_provider_state_storage",
                "resolve_provider_state_storage",
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
            FrameworkDependency::Provider(trusted_issuer_item_path(
                &issuer.trust.tag,
                TrustedIssuerItem::TrustedIssuer,
            )),
            FrameworkDependency::SingletonView(declaring.clone()),
        ],
        ConstructorOutcome::Infallible,
        FrameworkEnablement::Dependency,
        subject_token_exchanger_path(&issuer.trust.tag),
    )
}

fn aggregate(aggregate: ProviderAggregate) -> FrameworkDependency {
    FrameworkDependency::Constant(provider_aggregate_path(aggregate))
}

pub(crate) fn oidc_provider_framework_providers(
    registered_clients: Vec<CanonicalPath>,
    exchangers: &[SubjectTokenExchangerBinding],
) -> Vec<FrameworkProvider> {
    let state = || FrameworkDependency::Provider(provider_state_canonical_path());
    let secret_store = || FrameworkDependency::Provider(server_secret_store_canonical_path());
    let mut providers = vec![
        provider_state_storage(),
        constructed(
            vec![
                FrameworkDependency::Providers(registered_clients),
                FrameworkDependency::TokenIssuance,
            ],
            ConstructorOutcome::Infallible,
            FrameworkEnablement::Dependency,
            oidc_provider_item_path(OidcProviderItem::AcceptedClients),
        ),
        constructed(
            vec![FrameworkDependency::Providers(
                exchangers
                    .iter()
                    .map(|binding| subject_token_exchanger_path(&binding.issuer.trust.tag))
                    .collect(),
            )],
            ConstructorOutcome::Infallible,
            FrameworkEnablement::Dependency,
            oidc_provider_item_path(OidcProviderItem::SubjectTokenExchangers),
        ),
        endpoint(
            vec![
                item(OidcProviderItem::AcceptedClients),
                state(),
                FrameworkDependency::Constant(provider_endpoints_path()),
                FrameworkDependency::TokenIssuance,
            ],
            OidcProviderItem::AuthorizationEndpoint,
        ),
        endpoint(
            vec![state(), FrameworkDependency::TokenIssuance],
            OidcProviderItem::ConsentEndpoint,
        ),
        endpoint(
            vec![
                item(OidcProviderItem::AcceptedClients),
                secret_store(),
                state(),
            ],
            OidcProviderItem::IntrospectionEndpoint,
        ),
        constructed(
            vec![
                aggregate(ProviderAggregate::ProviderSupport),
                FrameworkDependency::Constant(provider_endpoints_path()),
                FrameworkDependency::TokenIssuance,
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
                aggregate(ProviderAggregate::AcceptedResources),
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
            vec![secret_store(), FrameworkDependency::TokenIssuance],
            OidcProviderItem::UserinfoEndpoint,
        ),
    ];

    providers.extend(exchangers.iter().map(subject_token_exchanger));

    providers
}
