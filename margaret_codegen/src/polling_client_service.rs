use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::container_bindings::ContainerBindings;
use margaret_service_codegen::framework_service::FrameworkService;
use margaret_service_codegen::framework_service_kind::FrameworkServiceKind;

pub(crate) fn polling_client_service(
    bindings: &ContainerBindings,
    client: CanonicalPath,
) -> Option<FrameworkService> {
    bindings.provider(&client).map(|binding| FrameworkService {
        concrete_path: client,
        field_name: binding.field_name.clone(),
        is_async: true,
        kind: FrameworkServiceKind::Service,
        runner: "run".to_string(),
        takes_token: true,
        type_name: binding.type_name.clone(),
    })
}
