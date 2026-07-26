use crate::authenticated_user_login_route::AuthenticatedUserLoginRoute;
use crate::authenticated_user_provider::AuthenticatedUserProvider;

pub struct AuthenticatedUserWrapperBinding<'provider> {
    pub login_route: AuthenticatedUserLoginRoute,
    pub provider: &'provider AuthenticatedUserProvider,
}
