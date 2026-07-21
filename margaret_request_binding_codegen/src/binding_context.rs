use margaret_container::container_bindings::ContainerBindings;
use margaret_route_parameter_codegen::route_path::RoutePath;

pub enum BindingContext<'context> {
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
            Self::Handshake { subject, .. }
            | Self::Middleware { subject }
            | Self::Responder { subject, .. } => subject,
        }
    }
}
