use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::http_validation::request_input::RequestInput;
use margaret::framework::macros::handles_middleware_attribute;
use margaret::framework::macros::process;

use crate::forms::archive_visit::ArchiveVisit;

#[handles_middleware_attribute(attribute = archive_visit)]
pub struct ArchiveVisitCheck;

impl ArchiveVisitCheck {
    /// # Errors
    ///
    /// Returns an error when the archive itself fails to answer.
    #[process]
    pub async fn process(
        &self,
        request: &Request,
        #[form_request(from = RequestInput::Query)] _visit: ArchiveVisit,
        next: Next,
    ) -> anyhow::Result<ResponseContinuation> {
        next.run(request).await.map_err(anyhow::Error::from)
    }
}
