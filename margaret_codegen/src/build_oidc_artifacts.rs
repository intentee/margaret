use margaret_container::container_bindings::ContainerBindings;
use margaret_oidc_codegen::oidc_client_canonical_path::oidc_client_canonical_path;
use margaret_oidc_codegen::render_oidc::render_oidc;
use margaret_tag_codegen::segmented_tag_binding::SegmentedTagBinding;

use crate::framework_artifacts::FrameworkArtifacts;
use crate::polling_client_service::polling_client_service;

pub(crate) fn build_oidc_artifacts(
    bindings: &ContainerBindings,
    issuer_bindings: &[SegmentedTagBinding],
) -> FrameworkArtifacts {
    let segments: Vec<&str> = issuer_bindings
        .iter()
        .map(|binding| binding.module_segment.as_str())
        .collect();
    let services = issuer_bindings
        .iter()
        .filter_map(|binding| {
            polling_client_service(
                bindings,
                oidc_client_canonical_path(&binding.module_segment),
            )
        })
        .collect();
    let enabled = !segments.is_empty();
    let modules = if enabled {
        render_oidc(&segments)
    } else {
        Vec::new()
    };

    FrameworkArtifacts {
        enabled,
        modules,
        services,
    }
}
