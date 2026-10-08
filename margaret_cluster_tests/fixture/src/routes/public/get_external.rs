use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::models::external_caller::ExternalCaller;
use crate::routes::public::subject_answer::SubjectAnswer;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/external", server = "public")]
pub struct GetExternal;

impl GetExternal {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] ExternalCaller { subject }: ExternalCaller,
    ) -> anyhow::Result<Response> {
        Ok(Response::json(200, &SubjectAnswer { subject }))
    }
}
