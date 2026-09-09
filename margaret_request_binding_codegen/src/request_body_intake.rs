use crate::request_binding::RequestBinding;
use crate::request_binding_error::RequestBindingError;
use crate::request_input_source::RequestInputSource;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RequestBodyIntake {
    Collected,
    Ignored,
    Parsed,
}

impl RequestBodyIntake {
    /// # Errors
    ///
    /// Returns `RequestBindingError::ConflictingRequestBodyIntake` when one component collects
    /// the raw body while another parses it.
    pub fn combine(self, other: Self, subject: &str) -> Result<Self, RequestBindingError> {
        match (self, other) {
            (Self::Collected, Self::Parsed) | (Self::Parsed, Self::Collected) => {
                Err(RequestBindingError::ConflictingRequestBodyIntake {
                    subject: subject.to_string(),
                })
            }
            (Self::Collected, _) | (_, Self::Collected) => Ok(Self::Collected),
            (Self::Parsed, _) | (_, Self::Parsed) => Ok(Self::Parsed),
            (Self::Ignored, Self::Ignored) => Ok(Self::Ignored),
        }
    }

    pub(crate) fn of_binding(binding: &RequestBinding) -> Self {
        match binding {
            RequestBinding::RequestBody => Self::Collected,
            RequestBinding::CurrentRequest => Self::Parsed,
            RequestBinding::FormRequest { source, .. } => match source {
                RequestInputSource::Form | RequestInputSource::Json => Self::Parsed,
                RequestInputSource::Cookie | RequestInputSource::Query => Self::Ignored,
            },
            RequestBinding::AssetBag
            | RequestBinding::AuthenticatedUser { .. }
            | RequestBinding::BoundRouteParameter { .. }
            | RequestBinding::Forwarder
            | RequestBinding::Injectable { .. }
            | RequestBinding::Next
            | RequestBinding::PeerSpiffeId
            | RequestBinding::RouteParameterValue { .. }
            | RequestBinding::Routes
            | RequestBinding::Views => Self::Ignored,
        }
    }
}
