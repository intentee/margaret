use std::collections::HashSet;

use proc_macro2::Ident;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::bound_parameter::BoundParameter;
use crate::classified_parameters::ClassifiedParameters;
use crate::request_binding::RequestBinding;
use crate::request_binding_error::RequestBindingError;
use crate::request_body_intake::RequestBodyIntake;

pub(crate) struct ClassifiedParametersBuilder<'subject> {
    authenticated_user_models: HashSet<CanonicalPath>,
    body_intake: RequestBodyIntake,
    has_next: bool,
    has_peer_spiffe_id: bool,
    has_request_body: bool,
    parameters: Vec<BoundParameter>,
    route_parameters: HashSet<String>,
    subject: &'subject str,
}

impl<'subject> ClassifiedParametersBuilder<'subject> {
    pub(crate) fn new(subject: &'subject str) -> Self {
        Self {
            authenticated_user_models: HashSet::new(),
            body_intake: RequestBodyIntake::Ignored,
            has_next: false,
            has_peer_spiffe_id: false,
            has_request_body: false,
            parameters: Vec::new(),
            route_parameters: HashSet::new(),
            subject,
        }
    }

    pub(crate) fn finish(self) -> ClassifiedParameters {
        ClassifiedParameters {
            body_intake: self.body_intake,
            parameters: self.parameters,
        }
    }

    /// # Errors
    ///
    /// Returns `RequestBindingError` when the binding conflicts with an earlier parameter.
    pub(crate) fn push(
        &mut self,
        binding: RequestBinding,
        holder: Ident,
    ) -> Result<(), RequestBindingError> {
        self.reserve_unique_binding(&binding)?;
        self.body_intake = self
            .body_intake
            .combine(RequestBodyIntake::of_binding(&binding), self.subject)?;
        self.parameters.push(BoundParameter { binding, holder });

        Ok(())
    }

    fn reserve_unique_binding(
        &mut self,
        binding: &RequestBinding,
    ) -> Result<(), RequestBindingError> {
        match binding {
            RequestBinding::AuthenticatedUser { application, .. } => {
                if !self
                    .authenticated_user_models
                    .insert(application.model.clone())
                {
                    return Err(RequestBindingError::MultipleAuthenticatedUserParameters {
                        subject: self.subject.to_string(),
                        model: application.model.to_string(),
                    });
                }
            }
            RequestBinding::BoundRouteParameter { path_key, .. }
            | RequestBinding::RouteParameterValue { path_key } => {
                if !self.route_parameters.insert(path_key.clone()) {
                    return Err(RequestBindingError::MultipleRouteParameterBindings {
                        subject: self.subject.to_string(),
                        parameter: path_key.clone(),
                    });
                }
            }
            RequestBinding::Next => {
                if self.has_next {
                    return Err(RequestBindingError::MultipleNextParameters {
                        subject: self.subject.to_string(),
                    });
                }
                self.has_next = true;
            }
            RequestBinding::PeerSpiffeId => {
                if self.has_peer_spiffe_id {
                    return Err(RequestBindingError::MultiplePeerSpiffeIdParameters {
                        subject: self.subject.to_string(),
                    });
                }
                self.has_peer_spiffe_id = true;
            }
            RequestBinding::RequestBody => {
                if self.has_request_body {
                    return Err(RequestBindingError::MultipleRequestBodyParameters {
                        subject: self.subject.to_string(),
                    });
                }
                self.has_request_body = true;
            }
            RequestBinding::AssetBag
            | RequestBinding::CurrentRequest
            | RequestBinding::FormRequest { .. }
            | RequestBinding::Forwarder
            | RequestBinding::Injectable { .. }
            | RequestBinding::Routes
            | RequestBinding::Views => {}
        }

        Ok(())
    }
}
