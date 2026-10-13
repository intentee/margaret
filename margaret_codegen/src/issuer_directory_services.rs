use margaret_container::container_bindings::ContainerBindings;
use margaret_service_codegen::framework_service::FrameworkService;
use margaret_service_codegen::runner_outcome::RunnerOutcome;

use crate::issuer_directory_canonical_path::issuer_directory_canonical_path;

pub(crate) fn issuer_directory_services(bindings: &ContainerBindings) -> Vec<FrameworkService> {
    let directory = issuer_directory_canonical_path();

    bindings
        .provider(&directory)
        .map(|binding| FrameworkService {
            concrete_path: directory.clone(),
            field_name: binding.field_name.clone(),
            is_async: true,
            outcome: RunnerOutcome::Infallible,
            runner: "run".to_string(),
            takes_token: true,
            type_name: binding.type_name.clone(),
        })
        .into_iter()
        .collect()
}
