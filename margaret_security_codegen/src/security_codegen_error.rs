use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;

#[derive(Debug, Error)]
pub enum SecurityCodegenError {
    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error(
        "more than one #[provides_authenticated_actor] store is defined: '{first}' and '{second}'; exactly one is allowed"
    )]
    AmbiguousAuthenticatedActorStore { first: String, second: String },

    #[error(
        "#[provides_authenticated_actor] store '{store}' has no `type Actor = <struct>` associated type that resolves to a known model"
    )]
    StoreUserUnresolved { store: String },

    #[error("#[decides_crud_action] gate '{gate}' has no #[decides] method")]
    CrudGateDecisionMissing { gate: String },

    #[error(
        "the #[decides] method of #[decides_crud_action] gate '{gate}' must take an `AuthenticatedActor` or `Option<AuthenticatedActor>` parameter"
    )]
    CrudGateActorMissing { gate: String },

    #[error(
        "the #[decides] method of #[decides_crud_action] gate '{gate}' must take a subject reference that resolves to a known model"
    )]
    CrudGateSubjectMissing { gate: String },

    #[error("model '{subject}' has more than one CRUD gate: '{first}' and '{second}'")]
    AmbiguousCrudActionGate {
        subject: String,
        first: String,
        second: String,
    },

    #[error("#[decides_site_action] gate '{gate}' has no #[decides] method")]
    SiteActionGateDecisionMissing { gate: String },

    #[error(
        "the #[decides] method of #[decides_site_action] gate '{gate}' must take an `AuthenticatedActor` or `Option<AuthenticatedActor>` parameter"
    )]
    SiteActionGateActorMissing { gate: String },

    #[error("#[decides_site_action] gate '{gate}' is missing its site action argument")]
    MissingSiteActionArgument { gate: String },

    #[error(
        "#[decides_site_action] gate '{gate}' names site action '{written}', which is not a fully qualified `Enum::Variant` path"
    )]
    MalformedSiteActionPath { gate: String, written: String },

    #[error("site action '{action}' has more than one gate: '{first}' and '{second}'")]
    AmbiguousSiteActionGate {
        action: String,
        first: String,
        second: String,
    },

    #[error(
        "#[decides_site_action] gate '{gate}' decides for site action enum '{action_type}', but '{expected}' is already in use; all site actions must share one enum"
    )]
    MismatchedSiteActionType {
        gate: String,
        action_type: String,
        expected: String,
    },
}
