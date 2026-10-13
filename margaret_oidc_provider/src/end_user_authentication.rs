use crate::authenticated_end_user::AuthenticatedEndUser;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndUserAuthentication {
    Anonymous,
    Authenticated(AuthenticatedEndUser),
}
