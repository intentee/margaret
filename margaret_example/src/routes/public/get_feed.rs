use margaret_http::response::Response;
use margaret_macros::process;
use margaret_macros::responds_to_http;
use margaret_macros::singleton;

use crate::models::user::User;

#[singleton]
#[responds_to_http(method = "get", name = "get_feed", path = "/feed", server = "public")]
pub struct GetFeed;

impl GetFeed {
    #[process]
    pub async fn respond(&self, #[authenticated_user] reader: Option<User>) -> Response {
        match reader {
            Some(reader) => Response::text(200, format!("your feed, {}", reader.name)),
            None => Response::text(200, "the public feed".to_string()),
        }
    }
}
