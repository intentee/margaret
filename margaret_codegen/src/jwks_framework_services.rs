use margaret_container::container_bindings::ContainerBindings;
use margaret_jwks_codegen::jwks_roll_interval_path::jwks_roll_interval_path;
use margaret_service_codegen::framework_service::FrameworkService;
use margaret_service_codegen::framework_service_kind::FrameworkServiceKind;
use margaret_tag_codegen::segmented_tag_binding::SegmentedTagBinding;

use crate::jwks_client_canonical_path::jwks_client_canonical_path;
use crate::jwks_roller_canonical_path::jwks_roller_canonical_path;
use crate::polling_client_service::polling_client_service;

pub(crate) fn jwks_framework_services(
    bindings: &ContainerBindings,
    client_bindings: &[SegmentedTagBinding],
) -> Vec<FrameworkService> {
    let roller = jwks_roller_canonical_path();
    let mut services = Vec::new();

    services.extend(bindings.provider(&roller).map(|binding| FrameworkService {
        concrete_path: roller.clone(),
        field_name: binding.field_name.clone(),
        is_async: false,
        kind: FrameworkServiceKind::Ticker {
            interval: jwks_roll_interval_path(),
        },
        runner: "run".to_string(),
        takes_token: false,
        type_name: binding.type_name.clone(),
    }));

    for binding in client_bindings {
        services.extend(polling_client_service(
            bindings,
            jwks_client_canonical_path(&binding.module_segment),
        ));
    }

    services
}
