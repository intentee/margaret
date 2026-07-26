use thiserror::Error;

use margaret_attributes::attribute_error::AttributeError;

#[derive(Debug, Error)]
pub enum RequestBindingError {
    #[error("failed to read a binding attribute: {source}")]
    Attribute {
        #[from]
        source: AttributeError,
    },

    #[error(
        "parameter '{parameter}' of {subject} must be a route parameter, a form request, the current request, the peer SPIFFE id, the forwarder, or the routes"
    )]
    UnmarkedParameter { subject: String, parameter: String },

    #[error(
        "parameter '{parameter}' of {subject} must be the current request, the next handler, a form request, the peer SPIFFE id, the views, an asset bag, or the routes"
    )]
    UnmarkedMiddlewareParameter { subject: String, parameter: String },

    #[error(
        "parameter '{parameter}' of {subject} is the next handler, which is only available inside an HTTP middleware"
    )]
    NextOutsideMiddleware { subject: String, parameter: String },

    #[error("{subject} declares more than one next handler; a middleware forwards to exactly one")]
    MultipleNextParameters { subject: String },

    #[error(
        "parameter '{parameter}' of {subject} is not an injectable dependency; expected Arc<T> or Vec<Arc<dyn Trait>>"
    )]
    UnsupportedParameterShape { subject: String, parameter: String },

    #[error(
        "parameter '{parameter}' of {subject} requests an injectable dependency, but no #[singleton] provides it"
    )]
    MissingProvider { subject: String, parameter: String },

    #[error(
        "route parameter #{parameter} of {subject} is missing `from = \"...\"`; it must name the path parameter it binds"
    )]
    RouteParameterMissingFrom { subject: String, parameter: String },

    #[error(
        "route parameter '{parameter}' of {subject} does not appear in the route path '{path}'"
    )]
    RouteParameterNotInPath {
        subject: String,
        parameter: String,
        path: String,
    },

    #[error(
        "argument #{parameter} of {subject} carries #[route_parameter], but {subject} has no route path to bind from"
    )]
    RouteParameterUnavailable { subject: String, parameter: String },

    #[error(
        "route parameter '{parameter}' of {subject} has type '{written}', which has no #[provides_route_parameter]"
    )]
    MissingRouteParameterBinder {
        subject: String,
        parameter: String,
        written: String,
    },

    #[error(
        "form request argument #{parameter} of {subject} is missing `from = <source>`; it must name the request input source it validates"
    )]
    FormRequestMissingSource { subject: String, parameter: String },

    #[error(
        "form request argument #{parameter} of {subject} names an unknown request input source '{written}'; expected Form, Query, Json, or Cookie"
    )]
    UnknownRequestInput {
        subject: String,
        parameter: String,
        written: String,
    },

    #[error(
        "form request argument #{parameter} of {subject} reads the request body via '{input_source}', but a WebSocket upgrade handshake has no body; only `from = Query` or `from = Cookie` is available"
    )]
    FormRequestBodyUnavailable {
        subject: String,
        parameter: String,
        input_source: String,
    },

    #[error(
        "argument #{parameter} of {subject} has both #[route_parameter] and #[form_request]; an argument may use at most one"
    )]
    ConflictingArgumentMarkers { subject: String, parameter: String },

    #[error(
        "argument #{parameter} of {subject} is the peer SPIFFE id and must not also carry #[authenticated_user], #[route_parameter], or #[form_request]"
    )]
    MarkedPeerSpiffeIdParameter { subject: String, parameter: String },

    #[error(
        "{subject} declares more than one peer SPIFFE id parameter; a request has exactly one peer identity"
    )]
    MultiplePeerSpiffeIdParameters { subject: String },

    #[error(
        "argument #{parameter} of {subject} requests the forwarder, but a WebSocket session builder cannot forward a request"
    )]
    ForwarderUnavailable { subject: String, parameter: String },

    #[error(
        "#[provides_route_parameter] '{binder}' has no `type Model = <struct>` associated type that resolves to a known model"
    )]
    RouteParameterBinderModel { binder: String },

    #[error(
        "#[provides_route_parameter] is only supported on structs, but '{binder}' is not a struct"
    )]
    RouteParameterBinderNotAStruct { binder: String },

    #[error("model '{model}' has more than one route parameter binder: '{first}' and '{second}'")]
    AmbiguousRouteParameterBinder {
        model: String,
        first: String,
        second: String,
    },

    #[error("#[provides_route_parameter] '{binder}' must also be declared as a #[singleton]")]
    RouteParameterBinderRequiresSingleton { binder: String },

    #[error(
        "#[infers_authenticated_user] is only supported on structs, but '{provider}' is not a struct"
    )]
    AuthenticatedUserProviderNotAStruct { provider: String },

    #[error("#[infers_authenticated_user] '{provider}' must also be declared as a #[singleton]")]
    AuthenticatedUserProviderRequiresSingleton { provider: String },

    #[error(
        "#[infers_authenticated_user] '{provider}' is missing `user_model = <struct>`; it must name the user model it infers"
    )]
    AuthenticatedUserProviderMissingUserModel { provider: String },

    #[error(
        "#[infers_authenticated_user] '{provider}' names the user model '{written}', which matches no struct"
    )]
    AuthenticatedUserProviderUnknownUserModel { provider: String, written: String },

    #[error("#[infers_authenticated_user] '{provider}' has no #[infer_from_request] method")]
    MissingInferFromRequest { provider: String },

    #[error(
        "#[infers_authenticated_user] '{provider}' has more than one #[infer_from_request] method: {methods}"
    )]
    AmbiguousInferFromRequest { provider: String, methods: String },

    #[error(
        "the #[infer_from_request] method of '{provider}' returns '{written}'; it must return anyhow::Result<margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome<Model>>"
    )]
    InferenceReturnTypeMismatch { provider: String, written: String },

    #[error(
        "the #[infer_from_request] method of '{provider}' infers '{written}', but the provider declares the user model '{model}'"
    )]
    InferredUserModelMismatch {
        provider: String,
        model: String,
        written: String,
    },

    #[error(
        "user model '{model}' has more than one authenticated user provider: '{first}' and '{second}'"
    )]
    AmbiguousAuthenticatedUserProvider {
        model: String,
        first: String,
        second: String,
    },

    #[error(
        "argument #{parameter} of {subject} carries #[authenticated_user] on '{written}', which matches no struct"
    )]
    UnknownAuthenticatedUserModel {
        subject: String,
        parameter: String,
        written: String,
    },

    #[error(
        "argument #{parameter} of {subject} requests the authenticated user '{model}', which no #[infers_authenticated_user] provides"
    )]
    MissingAuthenticatedUserProvider {
        subject: String,
        parameter: String,
        model: String,
    },

    #[error(
        "argument #{parameter} of {subject} requests the authenticated user by reference; it must be taken by value, optionally wrapped in Option<..>"
    )]
    AuthenticatedUserByReference { subject: String, parameter: String },

    #[error(
        "argument #{parameter} of {subject} carries #[authenticated_user] together with #[route_parameter] or #[form_request]; an argument may use at most one"
    )]
    ConflictingAuthenticatedUserMarkers { subject: String, parameter: String },

    #[error(
        "{subject} requests the authenticated user '{model}' more than once; a request infers it exactly once"
    )]
    MultipleAuthenticatedUserParameters { subject: String, model: String },

    #[error(
        "argument #{parameter} of {subject} carries #[authenticated_user], which is only available in an HTTP responder or a WebSocket session builder; let the provider return AuthenticatedUserOutcome::LoginPageRedirect to gate a request elsewhere"
    )]
    AuthenticatedUserUnavailable { subject: String, parameter: String },

    #[error(
        "argument #{parameter} of {subject} requests the authenticated user from '{provider}', which reads the request body via '{input_source}', but a WebSocket upgrade handshake has no body"
    )]
    AuthenticatedUserBodyUnavailable {
        subject: String,
        parameter: String,
        provider: String,
        input_source: String,
    },

    #[error(
        "parameter '{parameter}' of {subject} must be the current request, a form request, the peer SPIFFE id, the views, an asset bag, or the routes"
    )]
    UnmarkedProviderParameter { subject: String, parameter: String },

    #[error(
        "argument #{parameter} of {subject} requests the authenticated user from '{provider}', which renders the views, but a WebSocket upgrade handshake has no views"
    )]
    AuthenticatedUserViewsUnavailable {
        subject: String,
        parameter: String,
        provider: String,
    },

    #[error(
        "parameter '{parameter}' of {subject} requests the views, but a WebSocket upgrade handshake has no views; render them from an HTTP responder instead"
    )]
    ViewsUnavailableInHandshake { subject: String, parameter: String },

    #[error(
        "parameter '{parameter}' of {subject} requests the views, but this crate generates none; the views are generated for a crate that declares #[renders_view] and serves at least one #[responds_to_http] responder"
    )]
    ViewsUnavailable { subject: String, parameter: String },
}
