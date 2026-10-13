use margaret_http_codegen::http_server::HttpServer;
use margaret_http_codegen::serves_spiffe::serves_spiffe;

use crate::service_plan::ServicePlan;

#[derive(Clone, Copy)]
pub(crate) struct SpiffeActivation {
    pub(crate) client_active: bool,
    pub(crate) server_active: bool,
}

impl SpiffeActivation {
    pub(crate) fn of(plan: &ServicePlan, servers: &[HttpServer]) -> Self {
        Self {
            client_active: plan.spiffe_clients.any(),
            server_active: serves_spiffe(servers),
        }
    }

    pub(crate) fn any(self) -> bool {
        self.client_active || self.server_active
    }
}
