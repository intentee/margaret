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
}
