use std::env::VarError;
use std::io;
use std::path::PathBuf;

use thiserror::Error;

use margaret_accepted_clients_codegen::accepted_clients_codegen_error::AcceptedClientsCodegenError;
use margaret_asset_bag_codegen::asset_bag_codegen_error::AssetBagCodegenError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_console_codegen::console_codegen_error::ConsoleCodegenError;
use margaret_container::container_error::ContainerError;
use margaret_generated_module::generated_module_error::GeneratedModuleError;
use margaret_http_codegen::http_codegen_error::HttpCodegenError;
use margaret_middleware_codegen::middleware_codegen_error::MiddlewareCodegenError;
use margaret_model_codegen::model_codegen_error::ModelCodegenError;
use margaret_oauth_client_codegen::oauth_client_codegen_error::OAuthClientCodegenError;
use margaret_oidc_provider_codegen::oidc_provider_codegen_error::OidcProviderCodegenError;
use margaret_request_binding_codegen::request_binding_error::RequestBindingError;
use margaret_serve_input_codegen::serve_input_codegen_error::ServeInputCodegenError;
use margaret_service_codegen::service_codegen_error::ServiceCodegenError;
use margaret_tag_codegen::tag_error::TagError;
use margaret_token_issuance_codegen::token_issuance_codegen_error::TokenIssuanceCodegenError;
use margaret_trusted_issuer_codegen::trusted_issuer_codegen_error::TrustedIssuerCodegenError;
use margaret_views_codegen::views_codegen_error::ViewsCodegenError;
use margaret_websocket_codegen::web_socket_codegen_error::WebSocketCodegenError;

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
    },

    #[error("failed to generate the asset bag: {source}")]
    AssetBag {
        #[from]
        source: AssetBagCodegenError,
    },

    #[error("failed to read the token issuance: {source}")]
    TokenIssuance {
        #[from]
        source: TokenIssuanceCodegenError,
    },

    #[error("failed to read the accepted oauth clients: {source}")]
    AcceptedClients {
        #[from]
        source: AcceptedClientsCodegenError,
    },

    #[error("failed to read the oauth clients: {source}")]
    OAuthClient {
        #[from]
        source: OAuthClientCodegenError,
    },

    #[error("failed to read the trusted issuers: {source}")]
    TrustedIssuer {
        #[from]
        source: TrustedIssuerCodegenError,
    },

    #[error("failed to read the serve inputs: {source}")]
    ServeInput {
        #[from]
        source: ServeInputCodegenError,
    },

    #[error(
        "a constructor injects the asset responder, but no esbuild metafile was found in the manifest directory"
    )]
    AssetResponderWithoutMetafile,

    #[error(
        "a module imports the asset macro, but no esbuild metafile was found in the manifest directory"
    )]
    AssetMacroWithoutMetafile,

    #[error("failed to derive the openid connect provider endpoints: {source}")]
    OidcProvider {
        #[from]
        source: OidcProviderCodegenError,
    },

    #[error("failed to collect the tags: {source}")]
    Tag {
        #[from]
        source: TagError,
    },

    #[error("failed to generate the dependency container: {source}")]
    Container {
        #[from]
        source: ContainerError,
    },

    #[error("failed to generate the http server: {source}")]
    Http {
        #[from]
        source: HttpCodegenError,
    },

    #[error("failed to bind the request parameters: {source}")]
    RequestBinding {
        #[from]
        source: RequestBindingError,
    },

    #[error("failed to generate the middleware: {source}")]
    Middleware {
        #[from]
        source: MiddlewareCodegenError,
    },

    #[error("failed to generate the console: {source}")]
    Console {
        #[from]
        source: ConsoleCodegenError,
    },

    #[error("failed to generate the services: {source}")]
    Services {
        #[from]
        source: ServiceCodegenError,
    },

    #[error("failed to generate the models: {source}")]
    Model {
        #[from]
        source: ModelCodegenError,
    },

    #[error("failed to generate the views: {source}")]
    Views {
        #[from]
        source: ViewsCodegenError,
    },

    #[error("failed to generate the websockets: {source}")]
    Websocket {
        #[from]
        source: WebSocketCodegenError,
    },

    #[error("failed to format a generated module: {source}")]
    Format {
        #[from]
        source: GeneratedModuleError,
    },

    #[error("the CARGO_MANIFEST_DIR environment variable is not available: {source}")]
    ManifestDirectory {
        #[source]
        source: VarError,
    },

    #[error("failed to create the generated directory '{path}': {source}")]
    CreateDirectory { path: PathBuf, source: io::Error },

    #[error("failed to read the generated directory '{path}': {source}")]
    ReadDirectory { path: PathBuf, source: io::Error },

    #[error("failed to read the esbuild metafile '{path}': {source}")]
    ReadMetafile { path: PathBuf, source: io::Error },

    #[error("failed to read the asset directory '{path}': {source}")]
    ReadAssetDirectory { path: PathBuf, source: io::Error },

    #[error("the asset file name '{path}' is not valid UTF-8")]
    NonUtf8AssetPath { path: PathBuf },

    #[error(
        "the asset '{path}' is a symbolic link, which is not allowed under the asset directory"
    )]
    SymlinkAsset { path: PathBuf },

    #[error("the asset '{path}' is not a regular file")]
    NonRegularAsset { path: PathBuf },

    #[error("failed to write the generated source '{path}': {source}")]
    WriteSource { path: PathBuf, source: io::Error },

    #[error("failed to remove the stale generated entry '{path}': {source}")]
    RemoveEntry { path: PathBuf, source: io::Error },
}
