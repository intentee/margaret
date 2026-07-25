use heck::ToUpperCamelCase;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_jwks_codegen::jwks_roll_interval_path::jwks_roll_interval_path;
use margaret_service_codegen::framework_service::FrameworkService;
use margaret_service_codegen::framework_service_kind::FrameworkServiceKind;

use crate::jwks_client_path::jwks_client_canonical_path;
use crate::jwks_roller_path::jwks_roller_canonical_path;

fn framework_adapter_name(path: &CanonicalPath) -> String {
    path.segments()
        .iter()
        .map(|segment| segment.to_upper_camel_case())
        .collect()
}

pub(crate) fn jwks_framework_services(bindings: &ContainerBindings) -> Vec<FrameworkService> {
    let roller = jwks_roller_canonical_path();
    let client = jwks_client_canonical_path();
    let mut services = Vec::new();

    if bindings.provides(&roller) {
        let type_name = framework_adapter_name(&roller);

        services.push(FrameworkService {
            concrete_path: roller,
            kind: FrameworkServiceKind::Ticker {
                interval: jwks_roll_interval_path(),
            },
            runner: "run".to_string(),
            takes_token: false,
            type_name,
        });
    }

    if bindings.provides(&client) {
        let type_name = framework_adapter_name(&client);

        services.push(FrameworkService {
            concrete_path: client,
            kind: FrameworkServiceKind::Service,
            runner: "run".to_string(),
            takes_token: true,
            type_name,
        });
    }

    services
}
