use margaret_container::container_bindings::ContainerBindings;
use margaret_service_codegen::framework_service::FrameworkService;
use margaret_service_codegen::framework_service_kind::FrameworkServiceKind;
use margaret_service_codegen::runner_outcome::RunnerOutcome;
use margaret_tag_codegen::trusted_issuer_binding::TrustedIssuerBinding;
use margaret_trusted_issuer_codegen::render_trusted_issuers::render_trusted_issuers;

use crate::framework_artifacts::FrameworkArtifacts;
use crate::issuer_directory_canonical_path::issuer_directory_canonical_path;

pub(crate) fn build_trusted_issuer_artifacts(
    bindings: &ContainerBindings,
    trusted_issuer_bindings: &[TrustedIssuerBinding],
) -> FrameworkArtifacts {
    let directory = issuer_directory_canonical_path();
    let enabled = !trusted_issuer_bindings.is_empty();

    FrameworkArtifacts {
        enabled,
        modules: if enabled {
            render_trusted_issuers(trusted_issuer_bindings)
        } else {
            Vec::new()
        },
        services: bindings
            .provider(&directory)
            .map(|binding| FrameworkService {
                concrete_path: directory.clone(),
                field_name: binding.field_name.clone(),
                is_async: true,
                kind: FrameworkServiceKind::Service,
                outcome: RunnerOutcome::Infallible,
                runner: "run".to_string(),
                takes_token: true,
                type_name: binding.type_name.clone(),
            })
            .into_iter()
            .collect(),
    }
}
