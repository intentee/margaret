use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::injected_dependency::InjectedDependency;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
use crate::form_request_extraction::FormRequestExtraction;
use crate::head_input_source::HeadInputSource;

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
    UploadedFiles,
    Views,
}
