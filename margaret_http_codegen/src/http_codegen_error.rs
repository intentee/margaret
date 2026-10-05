use matchit::InsertError;
use thiserror::Error;

use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_container::container_error::ContainerError;
use margaret_injection_codegen::injection_error::InjectionError;
use margaret_middleware_codegen::middleware_codegen_error::MiddlewareCodegenError;
use margaret_request_binding_codegen::request_binding_error::RequestBindingError;
use margaret_route_method::route_method::RouteMethod;

#[derive(Debug, Error)]
pub enum HttpCodegenError {
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

    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error(transparent)]
    Injection {
        #[from]
        source: InjectionError,
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

    #[error("#[responds_to_http] is only supported on structs, but '{target}' is not a struct")]
    RespondsToHttpNotOnStruct { target: String },

    #[error("responder '{responder}' is missing the 'method' argument")]
    MissingHttpMethod { responder: String },

    #[error(
        "responder '{responder}' declares the HTTP method '{method}'; a route responds to one of \"get\", \"post\", \"put\", \"delete\", \"patch\" or \"query\""
    )]
    UnsupportedHttpMethod { responder: String, method: String },

    #[error(
        "responder '{responder}' responds to GET and reads the request body; a GET request carries no content a route may read"
    )]
    ContentOnGetRoute { responder: String },

    #[error(
        "responder '{responder}' reads the request body but declares no `max_body_bytes`; a route that reads its body declares how large that body may be"
    )]
    MissingBodyLimit { responder: String },

    #[error(
        "responder '{responder}' declares `max_body_bytes` but does not read the request body; the limit belongs to the route that reads the body"
    )]
    UnusedBodyLimit { responder: String },

    #[error("responder '{responder}' is missing the 'path' argument")]
    MissingHttpPath { responder: String },

    #[error("responder '{responder}' has a malformed route path '{path}': {source}")]
    InvalidRoutePath {
        responder: String,
        path: String,
        #[source]
        source: InsertError,
    },

    #[error(
        "responder '{responder}' registers route path '{path}' on server '{server}', which conflicts with the already registered route path '{conflicting_path}'"
    )]
    ConflictingRoutePaths {
        conflicting_path: String,
        path: String,
        responder: String,
        server: String,
    },

    #[error(
        "websocket session '{session}' serves '{path}' on server '{server}', which conflicts with the already registered route path '{conflicting_path}'"
    )]
    ConflictingWebSocketPath {
        conflicting_path: String,
        path: String,
        server: String,
        session: String,
    },

    #[error("websocket session '{session}' has a malformed path '{path}': {source}")]
    InvalidWebSocketPath {
        path: String,
        session: String,
        #[source]
        source: InsertError,
    },

    #[error(
        "websocket session '{session}' serves '{path}' on server '{server}', where a responder already serves that path"
    )]
    WebSocketPathOfResponder {
        path: String,
        server: String,
        session: String,
    },

    #[error(
        "responder '{responder}' registers '{method:?} {path}' on server '{server}', which is already registered by responder '{existing_responder}'"
    )]
    DuplicateRoute {
        existing_responder: String,
        method: RouteMethod,
        path: String,
        responder: String,
        server: String,
    },

    #[error(
        "responders '{first}' and '{second}' both declare the route name '{name}'; each route name may identify at most one responder"
    )]
    DuplicateRouteName {
        name: String,
        first: String,
        second: String,
    },

    #[error(
        "responder '{responder}' declares the route name '{name}', which must be a snake_case identifier usable as a `routes` accessor"
    )]
    InvalidRouteName { name: String, responder: String },

    #[error("responder '{responder}' is missing the 'server' argument")]
    MissingHttpServer { responder: String },

    #[error(
        "responder '{responder}' names the server '{server}', which must be a snake_case identifier usable as a `routes` accessor"
    )]
    InvalidServerName { responder: String, server: String },
}
