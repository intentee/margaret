use margaret_attributes::canonical_path::CanonicalPath;

use crate::form_request_extraction::FormRequestExtraction;
use crate::request_input_source::RequestInputSource;

pub(crate) enum ResponderArgumentBinding {
    CurrentRequest,
    Forwarder,
    PeerSpiffeId,
    Routes,
    Raw {
        path_key: String,
    },
    Bound {
        binder: CanonicalPath,
        path_key: String,
    },
    FormRequest {
        source: RequestInputSource,
        extraction: FormRequestExtraction,
    },
}

impl ResponderArgumentBinding {
    pub(crate) fn path_key(&self) -> Option<&str> {
        match self {
            Self::Raw { path_key } | Self::Bound { path_key, .. } => Some(path_key),
            _ => None,
        }
    }
}
