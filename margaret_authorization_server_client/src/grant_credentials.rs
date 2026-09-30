use headers::Authorization;
use headers::authorization::Basic;

use crate::form_parameter::FormParameter;

pub(crate) enum GrantCredentials {
    AuthorizationHeader(Authorization<Basic>),
    BodyParameters(Vec<FormParameter>),
}
