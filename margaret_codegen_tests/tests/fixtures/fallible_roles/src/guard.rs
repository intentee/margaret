use failures as errors;

use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;
use margaret::framework::http::response_continuation::ResponseContinuation;
use margaret::framework::macros::handles_middleware_attribute;
use margaret::framework::macros::process;

#[handles_middleware_attribute(attribute = guarded)]
pub struct Guard;

impl Guard {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub async fn process(
        &self,
        request: &Request,
        next: Next,
    ) -> errors::Result<ResponseContinuation> {
        Ok(next.run(request).await?)
    }
}
