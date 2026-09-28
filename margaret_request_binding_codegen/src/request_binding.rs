use margaret_attributes::canonical_path::CanonicalPath;
use margaret_container::injected_dependency::InjectedDependency;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
use crate::form_request_extraction::FormRequestExtraction;
use crate::request_input_source::RequestInputSource;

pub enum RequestBinding {
    AssetBag,
    AuthenticatedUser {
        application: AuthenticatedUserApplication,
        requirement: AuthenticatedUserRequirement,
    },
    BearerToken {
        claims: CanonicalPath,
        issuer_client: InjectedDependency,
    },
    BoundRouteParameter {
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
    Next,
    PeerSpiffeId,
    RouteParameterValue {
        path_key: String,
    },
    Routes,
    Views,
}
