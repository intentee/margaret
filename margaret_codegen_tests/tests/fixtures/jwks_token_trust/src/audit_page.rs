use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::partner_auditor::PartnerAuditor;

#[singleton]
#[responds_to_http(method = "get", path = "/audit", server = "public")]
pub struct AuditPage;

impl AuditPage {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] auditor: PartnerAuditor,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(200, auditor.scope))
    }
}
