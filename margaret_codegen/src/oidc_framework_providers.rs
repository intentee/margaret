use margaret_container::framework_dependency::FrameworkDependency;
use margaret_container::framework_injection_role::FrameworkInjectionRole;
use margaret_container::framework_provider::FrameworkProvider;
use margaret_oidc_codegen::oidc_client_canonical_path::oidc_client_canonical_path;
use margaret_tag_codegen::segmented_tag_binding::SegmentedTagBinding;

use crate::polling_client_provider::polling_client_provider;

pub(crate) fn oidc_framework_providers(
    issuer_bindings: &[SegmentedTagBinding],
) -> Vec<FrameworkProvider> {
    issuer_bindings
        .iter()
        .map(|binding| {
            polling_client_provider(
                oidc_client_canonical_path(&binding.module_segment),
                vec![FrameworkDependency::SingletonView(
                    binding.declaring.clone(),
                )],
                FrameworkInjectionRole::OidcClient(binding.tag.clone()),
            )
        })
        .collect()
}
