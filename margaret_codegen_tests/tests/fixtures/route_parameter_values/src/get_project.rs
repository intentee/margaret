use anyhow::Result;

use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use super::project_id::ProjectId;
use super::project_revision::ProjectRevision;
use super::project_slug::ProjectSlug;

#[singleton]
#[responds_to_http(
    method = "get",
    name = "get_project",
    path = "/projects/{project_slug}/{project_id}/{project_revision}",
    server = "public"
)]
pub struct GetProject;

impl GetProject {
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    #[process]
    pub fn respond(
        &self,
        #[route_parameter(from = "project_slug")] ProjectSlug(slug): ProjectSlug,
        #[route_parameter(from = "project_id")] ProjectId(id): ProjectId,
        #[route_parameter(from = "project_revision")] ProjectRevision(revision): ProjectRevision,
    ) -> Result<Response> {
        Ok(Response::text(200, format!("{slug}/{id}@{revision}")))
    }
}
