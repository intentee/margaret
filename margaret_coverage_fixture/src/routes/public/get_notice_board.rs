use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;
use margaret::framework::route_method::route_method::RouteMethod;

use crate::auth::authenticated_users::librarian::Librarian;

#[singleton]
#[responds_to_http(method = RouteMethod::Get, path = "/notice-board", server = "public")]
pub struct GetNoticeBoard;

impl GetNoticeBoard {
    /// # Errors
    ///
    /// Never returns an error; every visitor is answered.
    #[process]
    pub fn respond(
        &self,
        #[authenticated_user] librarian: Option<Librarian>,
    ) -> anyhow::Result<Response> {
        Ok(Response::text(
            200,
            match librarian {
                Some(Librarian { badge }) => format!("notices for badge {badge}"),
                None => "public notices".to_string(),
            },
        ))
    }
}
