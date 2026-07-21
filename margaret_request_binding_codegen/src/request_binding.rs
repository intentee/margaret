use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::injected_dependency::InjectedDependency;

use crate::form_request_extraction::FormRequestExtraction;
use crate::request_input_source::RequestInputSource;

pub enum RequestBinding {
    AssetBag,
    Bound {
        binder_field: String,
        binder_provider: CanonicalPath,
        path_key: String,
    },
    CurrentRequest,
    FormRequest {
        source: RequestInputSource,
        extraction: FormRequestExtraction,
    },
    Forwarder,
    Injectable {
        dependency: InjectedDependency,
    },
    PeerSpiffeId,
    Raw {
        path_key: String,
    },
    Routes,
    Views,
}

impl RequestBinding {
    #[must_use]
    pub fn path_key(&self) -> Option<&str> {
        match self {
            Self::Raw { path_key } | Self::Bound { path_key, .. } => Some(path_key),
            _ => None,
        }
    }

    #[must_use]
    pub fn references_request(&self) -> bool {
        matches!(
            self,
            Self::Raw { .. }
                | Self::Bound { .. }
                | Self::FormRequest { .. }
                | Self::PeerSpiffeId
                | Self::CurrentRequest
        )
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_container::injected_dependency::InjectedDependency;

    use crate::form_request_extraction::FormRequestExtraction;
    use crate::request_input_source::RequestInputSource;

    use super::RequestBinding;

    #[test]
    fn references_request_for_request_derived_bindings() {
        assert!(
            RequestBinding::Raw {
                path_key: "id".to_string(),
            }
            .references_request()
        );
        assert!(
            RequestBinding::Bound {
                binder_field: "user_binder".to_string(),
                binder_provider: CanonicalPath::new(vec![
                    "crate".to_string(),
                    "UserBinder".to_string(),
                ]),
                path_key: "id".to_string(),
            }
            .references_request()
        );
        assert!(
            RequestBinding::FormRequest {
                source: RequestInputSource::Query,
                extraction: FormRequestExtraction::Model,
            }
            .references_request()
        );
        assert!(RequestBinding::PeerSpiffeId.references_request());
        assert!(RequestBinding::CurrentRequest.references_request());
    }

    #[test]
    fn does_not_reference_request_for_container_or_generated_bindings() {
        assert!(
            !RequestBinding::Injectable {
                dependency: InjectedDependency::SingleConcrete {
                    concrete: CanonicalPath::new(vec!["crate".to_string(), "Service".to_string(),]),
                    field: "service".to_string(),
                },
            }
            .references_request()
        );
        assert!(!RequestBinding::AssetBag.references_request());
        assert!(!RequestBinding::Forwarder.references_request());
        assert!(!RequestBinding::Routes.references_request());
        assert!(!RequestBinding::Views.references_request());
    }
}
