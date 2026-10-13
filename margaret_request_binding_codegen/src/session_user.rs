use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::bound_parameter::BoundParameter;

pub struct SessionUser<'parameters> {
    pub application: &'parameters AuthenticatedUserApplication,
    pub parameter: &'parameters BoundParameter,
}
