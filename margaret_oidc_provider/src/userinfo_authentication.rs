use margaret_http::response::Response;

use crate::userinfo_grant::UserinfoGrant;

pub enum UserinfoAuthentication {
    Authenticated(UserinfoGrant),
    Refused(Response),
}
