use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_container::container_error::ContainerError;
use margaret_middleware_codegen::middleware_codegen_error::MiddlewareCodegenError;
use margaret_request_binding_codegen::request_binding_error::RequestBindingError;

#[derive(Debug, Error)]
pub enum WebSocketCodegenError {
    #[error("failed to read the attribute arguments: {source}")]
    AttributeArguments {
        #[from]
        source: AttributeArgumentsError,
    },

    #[error(transparent)]
    Container {
        #[from]
        source: ContainerError,
    },

    #[error("failed to read a websocket attribute: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error(transparent)]
    Binding {
        #[from]
        source: RequestBindingError,
    },

    #[error(transparent)]
    Middleware {
        #[from]
        source: MiddlewareCodegenError,
    },

    #[error("'{message}' carries #[websocket_message] but is not a struct")]
    MessageNotAStruct { message: String },

    #[error("'{message}' must declare a message kind: request, notification, or response")]
    MissingMessageKind { message: String },

    #[error("'{message}' declares more than one message kind")]
    InvalidMessageKind { message: String },

    #[error("'{message}' is missing the required 'method' argument")]
    MissingMethod { message: String },

    #[error("'{message}' has method '{method}', which is not a snake_case identifier")]
    InvalidMethod { message: String, method: String },

    #[error("request '{message}' must declare 'response = single' or 'response = stream'")]
    MissingCardinality { message: String },

    #[error("request '{message}' has an invalid cardinality; expected 'single' or 'stream'")]
    InvalidCardinality { message: String },

    #[error("'{message}' declares a response cardinality but is not a request")]
    CardinalityOnNonRequest { message: String },

    #[error("'{session}' carries #[websocket_session] but is not a struct")]
    SessionNotAStruct { session: String },

    #[error("session '{session}' is missing the required 'path' argument")]
    MissingSessionPath { session: String },

    #[error("session '{session}' is missing the required 'server' argument")]
    MissingSessionServer { session: String },

    #[error("session '{session}' has server '{server}', which is not a snake_case identifier")]
    InvalidSessionServer { session: String, server: String },

    #[error("session '{session}' has no #[build_for_session] method")]
    SessionMissingBuildForSession { session: String },

    #[error("session '{session}' has more than one #[build_for_session] method: {methods}")]
    AmbiguousBuildForSession { session: String, methods: String },

    #[error(
        "session '{session}' attaches the middleware '{middleware}', which renders the views, but a WebSocket upgrade handshake has no views"
    )]
    SessionMiddlewareRendersViews { session: String, middleware: String },

    #[error("'{handler}' implements a websocket handler trait but is not a #[singleton]")]
    HandlerNotSingleton { handler: String },

    #[error("'{handler}' implements a websocket handler trait but is not a struct")]
    HandlerNotAStruct { handler: String },

    #[error("'{handler}' is missing the '{associated_type}' associated type")]
    HandlerMissingAssociatedType {
        handler: String,
        associated_type: String,
    },

    #[error("the Session of handler '{handler}' does not resolve to a #[websocket_session]")]
    HandlerSessionNotASession { handler: String },

    #[error("the Message of handler '{handler}' does not resolve to a #[websocket_message]")]
    HandlerMessageNotAMessage { handler: String },

    #[error("handler '{handler}' handles a response message, which cannot be dispatched")]
    HandlerMessageIsResponse { handler: String },

    #[error("request handler '{handler}' handles '{message}', which is not a request message")]
    HandlerMessageNotARequest { handler: String, message: String },

    #[error(
        "notification handler '{handler}' handles '{message}', which is not a notification message"
    )]
    HandlerMessageNotANotification { handler: String, message: String },

    #[error("message '{message}' is handled by more than one handler: '{first}' and '{second}'")]
    DuplicateHandlerForMessage {
        message: String,
        first: String,
        second: String,
    },

    #[error("message '{message}' has no handler")]
    MessageWithoutHandler { message: String },

    #[error("session '{session}' binds method '{method}' more than once: '{first}' and '{second}'")]
    DuplicateMethodInSession {
        session: String,
        method: String,
        first: String,
        second: String,
    },

    #[error("response method '{method}' is declared more than once: '{first}' and '{second}'")]
    DuplicateResponseMethod {
        method: String,
        first: String,
        second: String,
    },
}
