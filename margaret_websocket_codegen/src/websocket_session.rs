use std::collections::BTreeMap;

use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_middleware_codegen::layer_application::LayerApplication;
use margaret_request_binding_codegen::authenticated_user_application::AuthenticatedUserApplication;
use margaret_request_binding_codegen::bound_parameter::BoundParameter;
use margaret_request_binding_codegen::injects_routes::injects_routes;
use margaret_request_binding_codegen::request_binding::RequestBinding;

pub(crate) struct WebSocketSession {
    pub(crate) layers: Vec<LayerApplication>,
    pub(crate) method_name: Ident,
    pub(crate) module_name: String,
    pub(crate) parameters: Vec<BoundParameter>,
    pub(crate) path: String,
    pub(crate) server: String,
    pub(crate) session_path: CanonicalPath,
}

impl WebSocketSession {
    pub(crate) fn authenticated_user_providers(&self) -> Vec<AuthenticatedUserApplication> {
        let mut providers: BTreeMap<String, AuthenticatedUserApplication> = BTreeMap::new();

        for parameter in &self.parameters {
            if let RequestBinding::AuthenticatedUser { application, .. } = &parameter.binding {
                providers.insert(application.field.clone(), application.clone());
            }
        }

        providers.into_values().collect()
    }

    pub(crate) fn injects_routes(&self) -> bool {
        injects_routes(&self.parameters) || !self.authenticated_user_providers().is_empty()
    }

    pub(crate) fn references_routes(&self) -> bool {
        self.injects_routes()
            || self
                .layers
                .iter()
                .any(|application| application.injects_routes)
    }
}
