use std::collections::HashSet;

use margaret_attributes::canonical_path::CanonicalPath;

use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::inference_channel::InferenceChannel;
use crate::request_binding::RequestBinding;
use crate::request_binding_error::RequestBindingError;

#[derive(Default)]
pub(crate) struct ExclusiveBindings {
    inference_channels: HashSet<InferenceChannel>,
    inferred_models: HashSet<CanonicalPath>,
    next: bool,
    peer_spiffe_id: bool,
}

impl ExclusiveBindings {
    pub(crate) fn admit(
        &mut self,
        binding: &RequestBinding,
        subject: &str,
    ) -> Result<(), RequestBindingError> {
        match binding {
            RequestBinding::Next if self.next => Err(RequestBindingError::MultipleNextParameters {
                subject: subject.to_string(),
            }),
            RequestBinding::Next => {
                self.next = true;

                Ok(())
            }
            RequestBinding::PeerSpiffeId if self.peer_spiffe_id => {
                Err(RequestBindingError::MultiplePeerSpiffeIdParameters {
                    subject: subject.to_string(),
                })
            }
            RequestBinding::PeerSpiffeId => {
                self.peer_spiffe_id = true;

                Ok(())
            }
            RequestBinding::AuthenticatedUser { application, .. } => {
                if !self.inferred_models.insert(application.model.clone()) {
                    return Err(RequestBindingError::MultipleAuthenticatedUserParameters {
                        subject: subject.to_string(),
                        model: application.model.to_string(),
                    });
                }

                match application.challenge {
                    AuthenticatedUserChallenge::Bearer { .. }
                    | AuthenticatedUserChallenge::Introspection { .. } => {
                        if self
                            .inference_channels
                            .insert(InferenceChannel::BearerCredential)
                        {
                            Ok(())
                        } else {
                            Err(RequestBindingError::MultipleBearerAuthenticatedUsers {
                                subject: subject.to_string(),
                            })
                        }
                    }
                    AuthenticatedUserChallenge::Session { .. } => {
                        if self.inference_channels.insert(InferenceChannel::Session) {
                            Ok(())
                        } else {
                            Err(RequestBindingError::MultipleSessionAuthenticatedUsers {
                                subject: subject.to_string(),
                            })
                        }
                    }
                    AuthenticatedUserChallenge::Unchallenged => Ok(()),
                }
            }
            RequestBinding::AssetBag
            | RequestBinding::BearerToken { .. }
            | RequestBinding::BoundRouteParameter { .. }
            | RequestBinding::CurrentRequest
            | RequestBinding::FormContent { .. }
            | RequestBinding::FormRequest { .. }
            | RequestBinding::Forwarder
            | RequestBinding::Injectable { .. }
            | RequestBinding::IntrospectedBearerToken { .. }
            | RequestBinding::JsonContent { .. }
            | RequestBinding::RequestBodyStream
            | RequestBinding::RouteParameterValue { .. }
            | RequestBinding::Routes
            | RequestBinding::Session { .. }
            | RequestBinding::UploadedFiles
            | RequestBinding::Views => Ok(()),
        }
    }
}
