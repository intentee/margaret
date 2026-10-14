use margaret_container::container_bindings::ContainerBindings;
use margaret_http_codegen::http_server::HttpServer;

use crate::read_serve_invocation::read_serve_invocation;
use crate::render_services::render_services;
use crate::rendered_serving::RenderedServing;
use crate::serve_invocation_fields::serve_invocation_fields;
use crate::service_plan::ServicePlan;
use crate::spiffe_activation::SpiffeActivation;

#[must_use]
pub fn render_serving(
    plan: &ServicePlan,
    servers: &[HttpServer],
    has_views: bool,
    bindings: &ContainerBindings,
) -> RenderedServing {
    let activation = SpiffeActivation::of(plan, servers);
    let fields = serve_invocation_fields(plan, servers, bindings, activation);

    RenderedServing {
        invocation: read_serve_invocation(plan, servers, activation, &fields),
        modules: render_services(plan, servers, has_views, bindings, activation, &fields),
    }
}
