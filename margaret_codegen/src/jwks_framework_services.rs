use heck::ToUpperCamelCase;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_jwks_codegen::jwks_roll_interval_path::jwks_roll_interval_path;
use margaret_service_codegen::framework_service::FrameworkService;
use margaret_service_codegen::framework_service_kind::FrameworkServiceKind;
use margaret_tag_codegen::jwks_client_binding::JwksClientBinding;

use crate::jwks_client_path::jwks_client_canonical_path;
use crate::jwks_roller_path::jwks_roller_canonical_path;

fn framework_service(
    bindings: &ContainerBindings,
    path: CanonicalPath,
    kind: FrameworkServiceKind,
    takes_token: bool,
) -> Option<FrameworkService> {
    bindings.accessor(&path).map(|field| FrameworkService {
        concrete_path: path,
        field_name: field.to_string(),
        kind,
        runner: "run".to_string(),
        takes_token,
        type_name: field.to_upper_camel_case(),
    })
}

pub(crate) fn jwks_framework_services(
    bindings: &ContainerBindings,
    client_bindings: &[JwksClientBinding],
) -> Vec<FrameworkService> {
    let mut services = Vec::new();

    services.extend(framework_service(
        bindings,
        jwks_roller_canonical_path(),
        FrameworkServiceKind::Ticker {
            interval: jwks_roll_interval_path(),
        },
        false,
    ));

    for binding in client_bindings {
        services.extend(framework_service(
            bindings,
            jwks_client_canonical_path(&binding.module_segment),
            FrameworkServiceKind::Service,
            true,
        ));
    }

    services
}
