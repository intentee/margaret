use proc_macro2::Ident;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::bound_parameter::BoundParameter;
use crate::inference_shape::InferenceShape;

pub struct AuthenticatedUserProvider {
    pub application: AuthenticatedUserApplication,
    pub declared: InferenceShape,
    pub is_async: bool,
    pub method_name: Ident,
    pub parameters: Vec<BoundParameter>,
}
