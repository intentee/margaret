use std::sync::Arc;

use margaret::framework::http::response::Response;
use margaret::framework::macros::constructor;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::singletons::audit_log::AuditLog;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/audits", server = "public")]
pub struct GetAudits {
    audit_log: Arc<AuditLog>,
}

impl GetAudits {
    #[must_use]
    #[constructor]
    pub fn create(audit_log: Arc<AuditLog>) -> Self {
        Self { audit_log }
    }

    /// # Errors
    ///
    /// Never returns an error; the audit count is always known.
    #[process]
    pub fn respond(&self) -> anyhow::Result<Response> {
        Ok(Response::text(200, self.audit_log.audits().to_string()))
    }
}
