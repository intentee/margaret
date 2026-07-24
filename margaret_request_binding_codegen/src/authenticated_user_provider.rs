use proc_macro2::Ident;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::bound_parameter::BoundParameter;

pub struct AuthenticatedUserProvider {
    pub application: AuthenticatedUserApplication,
    pub method_name: Ident,
    pub parameters: Vec<BoundParameter>,
}
