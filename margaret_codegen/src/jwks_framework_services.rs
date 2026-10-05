use margaret_container::container_bindings::ContainerBindings;
use margaret_jwks_codegen::jwks_roll_interval_path::jwks_roll_interval_path;
use margaret_service_codegen::first_tick::FirstTick;
use margaret_service_codegen::framework_service::FrameworkService;
use margaret_service_codegen::framework_service_kind::FrameworkServiceKind;
use margaret_service_codegen::runner_outcome::RunnerOutcome;

use crate::jwks_roller_canonical_path::jwks_roller_canonical_path;

pub(crate) fn jwks_framework_services(bindings: &ContainerBindings) -> Vec<FrameworkService> {
    let roller = jwks_roller_canonical_path();

    bindings
        .provider(&roller)
        .map(|binding| FrameworkService {
            concrete_path: roller.clone(),
            field_name: binding.field_name.clone(),
            is_async: false,
            kind: FrameworkServiceKind::Ticker {
                first_tick: FirstTick::AfterInterval,
                interval: jwks_roll_interval_path(),
            },
            outcome: RunnerOutcome::Fallible,
            runner: "run".to_string(),
            takes_token: false,
            type_name: binding.type_name.clone(),
        })
        .into_iter()
        .collect()
}
