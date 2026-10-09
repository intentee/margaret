use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::injected_dependency::InjectedDependency;
use margaret_tag_codegen::session_source::SessionSource;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
use crate::extraction_phase::ExtractionPhase;
use crate::form_request_extraction::FormRequestExtraction;
use crate::head_input_source::HeadInputSource;
use crate::route_parameter_lookup::RouteParameterLookup;

pub enum RequestBinding {
    AssetBag,
    AuthenticatedUser {
        application: AuthenticatedUserApplication,
        requirement: AuthenticatedUserRequirement,
    },
    BearerToken {
        claims: CanonicalPath,
        profile: CanonicalPath,
        trusted_issuer: InjectedDependency,
    },
    BoundRouteParameter {
        binder_field: String,
        binder_provider: CanonicalPath,
        lookup: RouteParameterLookup,
        path_key: String,
    },
    CurrentRequest,
    FormContent {
        extraction: FormRequestExtraction,
    },
    FormRequest {
        source: HeadInputSource,
        extraction: FormRequestExtraction,
    },
    Forwarder,
    Injectable {
        dependency: InjectedDependency,
    },
    IntrospectedBearerToken {
        authorization_server: InjectedDependency,
        claims: CanonicalPath,
    },
    JsonContent {
        extraction: FormRequestExtraction,
    },
    Next,
    PeerSpiffeId,
    RequestBodyStream,
    RouteParameterValue {
        path_key: String,
    },
    Routes,
    Session {
        sessions: InjectedDependency,
        source: SessionSource,
    },
    UploadedFiles,
    Views,
}

impl RequestBinding {
    pub(crate) fn extraction_phase(&self) -> ExtractionPhase {
        match self {
            Self::AuthenticatedUser { .. } | Self::PeerSpiffeId => ExtractionPhase::CallerIdentity,
            Self::BoundRouteParameter { .. } => ExtractionPhase::RouteModel,
            Self::AssetBag
            | Self::BearerToken { .. }
            | Self::CurrentRequest
            | Self::FormContent { .. }
            | Self::FormRequest { .. }
            | Self::Forwarder
            | Self::Injectable { .. }
            | Self::IntrospectedBearerToken { .. }
            | Self::JsonContent { .. }
            | Self::Next
            | Self::RequestBodyStream
            | Self::RouteParameterValue { .. }
            | Self::Routes
            | Self::Session { .. }
            | Self::UploadedFiles
            | Self::Views => ExtractionPhase::RequestInput,
        }
    }
}
