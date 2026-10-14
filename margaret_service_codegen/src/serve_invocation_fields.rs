use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::http_server::HttpServer;

use crate::serve_invocation_field::ServeInvocationField;
use crate::server_bindings::ServerBindings;
use crate::service_plan::ServicePlan;
use crate::spiffe_activation::SpiffeActivation;
use crate::svid_bundle_type::svid_bundle_type;

pub(crate) fn serve_invocation_fields(
    plan: &ServicePlan,
    servers: &[HttpServer],
    bindings: &ContainerBindings,
    activation: SpiffeActivation,
) -> Vec<ServeInvocationField> {
    let mut fields = Vec::new();

    if !plan.construction_arguments.is_empty() {
        fields.push(ServeInvocationField::Arguments {
            arguments_type: bindings.serve_arguments_type(),
            literal: bindings.serve_arguments(&plan.construction_arguments),
        });
    }

    if !servers.is_empty() {
        fields.push(ServeInvocationField::Listeners {
            listeners: servers
                .iter()
                .map(|server| ServerBindings::of(server).listener)
                .collect(),
        });
        fields.push(ServeInvocationField::Routes);
    }

    if activation.any() {
        fields.push(ServeInvocationField::SpiffeBundle {
            bundle_type: svid_bundle_type(activation),
        });
    }

    fields
}
