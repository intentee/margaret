use margaret::framework::http::response::Response;
use margaret::framework::macros::process;
use margaret::framework::macros::responds_to_http;
use margaret::framework::macros::singleton;

use crate::models::user::User;

#[singleton]
#[responds_to_http(method = "get", name = "get_feed", path = "/feed", server = "public")]
pub struct GetFeed;

impl GetFeed {
    #[process]
    /// # Errors
    ///
    /// Returns an error propagated from the work it performs.
    pub fn respond(&self, #[authenticated_user] reader: Option<User>) -> anyhow::Result<Response> {
        Ok({
            match reader {
                Some(reader) => Response::text(200, format!("your feed, {}", reader.name)),
                None => Response::text(200, "the public feed".to_string()),
            }
        })
    }
}
