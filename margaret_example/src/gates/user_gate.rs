use margaret_http::crud_action::CrudAction;
use margaret_http::crud_action_gate::CrudActionGate;
use margaret_http::request::Request;
use margaret_macros::crud_gate;
use margaret_macros::singleton;

use crate::models::user::User;

#[singleton]
#[crud_gate]
pub struct UserGate;

impl CrudActionGate for UserGate {
    type Subject = User;

    async fn can(&self, request: &Request, _subject: &User, _action: CrudAction) -> bool {
        request.header("x-authorized").is_some()
    }
}
