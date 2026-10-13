use margaret_container::container_bindings::ContainerBindings;
use margaret_service_codegen::framework_service::FrameworkService;
use margaret_service_codegen::runner_outcome::RunnerOutcome;

use crate::jwks_roller_canonical_path::jwks_roller_canonical_path;

pub(crate) fn jwks_framework_services(bindings: &ContainerBindings) -> Vec<FrameworkService> {
    let roller = jwks_roller_canonical_path();

    bindings
        .provider(&roller)
        .map(|binding| FrameworkService {
            concrete_path: roller.clone(),
            field_name: binding.field_name.clone(),
            is_async: true,
            outcome: RunnerOutcome::Fallible,
            runner: "run".to_string(),
            takes_token: true,
            type_name: binding.type_name.clone(),
        })
        .into_iter()
        .collect()
}
