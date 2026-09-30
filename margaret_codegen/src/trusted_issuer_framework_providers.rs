use margaret_container::constructor_outcome::ConstructorOutcome;
use margaret_container::framework_construction::FrameworkConstruction;
use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_enablement::FrameworkEnablement;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_tag_codegen::trusted_issuer_binding::TrustedIssuerBinding;
use margaret_tag_codegen::trusted_issuer_kind::TrustedIssuerKind;
use margaret_trusted_issuer_codegen::issuer_metadata_canonical_path::issuer_metadata_canonical_path;
use margaret_trusted_issuer_codegen::trusted_issuer_canonical_path::trusted_issuer_canonical_path;

use crate::issuer_directory_canonical_path::issuer_directory_canonical_path;
use crate::issuer_request_client_canonical_path::issuer_request_client_canonical_path;

fn trusted_issuer_providers(
    TrustedIssuerBinding {
        declaring,
        kind,
        module_segment,
        tag,
    }: &TrustedIssuerBinding,
) -> Vec<FrameworkProvider> {
    let trusted_issuer = |dependencies: Vec<FrameworkDependency>, method: &str| FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies,
            is_async: false,
            method: method.to_string(),
            outcome: ConstructorOutcome::Infallible,
        },
        enablement: FrameworkEnablement::Always,
        injection: FrameworkInjectionRole::TrustedIssuer(tag.clone()),
        provided: trusted_issuer_canonical_path(module_segment),
    };

    match kind {
        TrustedIssuerKind::JwksEndpoint => vec![trusted_issuer(
            vec![
                FrameworkDependency::SingletonView(declaring.clone()),
                FrameworkDependency::SingletonView(declaring.clone()),
            ],
            "for_jwks_endpoint",
        )],
        TrustedIssuerKind::OidcIssuer => vec![
            FrameworkProvider {
                construction: FrameworkConstruction::Constructor {
                    dependencies: Vec::new(),
                    is_async: false,
                    method: "awaiting".to_string(),
                    outcome: ConstructorOutcome::Infallible,
                },
                enablement: FrameworkEnablement::Dependency,
                injection: FrameworkInjectionRole::Unmarked,
                provided: issuer_metadata_canonical_path(module_segment),
            },
            trusted_issuer(
                vec![
                    FrameworkDependency::Provider(issuer_metadata_canonical_path(module_segment)),
                    FrameworkDependency::SingletonView(declaring.clone()),
                ],
                "for_oidc_issuer",
            ),
        ],
    }
}

pub(crate) fn trusted_issuer_framework_providers(
    trusted_issuer_bindings: &[TrustedIssuerBinding],
) -> Vec<FrameworkProvider> {
    if trusted_issuer_bindings.is_empty() {
        return Vec::new();
    }

    let mut providers: Vec<FrameworkProvider> = trusted_issuer_bindings
        .iter()
        .flat_map(trusted_issuer_providers)
        .collect();

    providers.push(FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: Vec::new(),
            is_async: false,
            method: "create".to_string(),
            outcome: ConstructorOutcome::Fallible,
        },
        enablement: FrameworkEnablement::Always,
        injection: FrameworkInjectionRole::Unmarked,
        provided: issuer_request_client_canonical_path(),
    });
    providers.push(FrameworkProvider {
        construction: FrameworkConstruction::Constructor {
            dependencies: vec![
                FrameworkDependency::Provider(issuer_request_client_canonical_path()),
                FrameworkDependency::Providers(
                    trusted_issuer_bindings
                        .iter()
                        .map(|binding| trusted_issuer_canonical_path(&binding.module_segment))
                        .collect(),
                ),
            ],
            is_async: false,
            method: "create".to_string(),
            outcome: ConstructorOutcome::Fallible,
        },
        enablement: FrameworkEnablement::Always,
        injection: FrameworkInjectionRole::Unmarked,
        provided: issuer_directory_canonical_path(),
    });

    providers
}
