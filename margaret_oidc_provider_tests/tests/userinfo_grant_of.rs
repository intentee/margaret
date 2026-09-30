use margaret_oidc_provider::userinfo_grant::UserinfoGrant;

use crate::end_user_subject::END_USER_SUBJECT;
use crate::fixture_scopes::fixture_scopes;

pub fn userinfo_grant_of() -> UserinfoGrant {
    UserinfoGrant {
        scopes: fixture_scopes(&["openid"]),
        subject: END_USER_SUBJECT,
    }
}
