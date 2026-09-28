use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_oidc_codegen::oidc_client_canonical_path::oidc_client_canonical_path;
use margaret_oidc_codegen::oidc_token_verifier_canonical_path::oidc_token_verifier_canonical_path;
use margaret_tag_codegen::segmented_tag_binding::SegmentedTagBinding;

use crate::polling_client_wiring::PollingClientWiring;

pub(crate) fn oidc_framework_providers(
    issuer_bindings: &[SegmentedTagBinding],
) -> Vec<FrameworkProvider> {
    issuer_bindings
        .iter()
        .flat_map(|binding| {
            PollingClientWiring {
                client: oidc_client_canonical_path(&binding.module_segment),
                client_dependencies: vec![FrameworkDependency::SingletonView(
                    binding.declaring.clone(),
                )],
                verifier: oidc_token_verifier_canonical_path(&binding.module_segment),
                verifier_injection: FrameworkInjectionRole::FrameworkOnly,
            }
            .providers()
        })
        .collect()
}
