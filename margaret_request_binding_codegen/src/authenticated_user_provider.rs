use proc_macro2::Ident;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::bound_parameter::BoundParameter;
use crate::request_body_intake::RequestBodyIntake;

pub struct AuthenticatedUserProvider {
    pub application: AuthenticatedUserApplication,
    pub body_intake: RequestBodyIntake,
    pub is_async: bool,
    pub method_name: Ident,
    pub parameters: Vec<BoundParameter>,
}
