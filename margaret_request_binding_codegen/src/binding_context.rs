use margaret_container::container_bindings::ContainerBindings;
use margaret_route_parameter_codegen::route_path::RoutePath;
use margaret_tag_codegen::tag_pool::TagPool;

pub enum BindingContext<'context> {
    AuthenticatedUserProvider {
        container_bindings: &'context ContainerBindings,
        subject: &'context str,
        tags: &'context TagPool<'context>,
    },
    Handshake {
        container_bindings: &'context ContainerBindings,
        route_path: &'context RoutePath,
        server: &'context str,
        subject: &'context str,
    },
    Middleware {
        subject: &'context str,
    },
    Responder {
        route_path: &'context RoutePath,
        server: &'context str,
        subject: &'context str,
    },
}

impl BindingContext<'_> {
    #[must_use]
    pub fn subject(&self) -> &str {
        match self {
            Self::AuthenticatedUserProvider { subject, .. }
            | Self::Handshake { subject, .. }
            | Self::Middleware { subject }
            | Self::Responder { subject, .. } => subject,
        }
    }
}
