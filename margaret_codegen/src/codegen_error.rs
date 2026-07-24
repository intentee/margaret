use std::path::PathBuf;

use thiserror::Error;

use margaret_asset_bag_codegen::asset_bag_codegen_error::AssetBagCodegenError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_console_argument_codegen::console_argument_codegen_error::ConsoleArgumentCodegenError;
use margaret_console_codegen::console_codegen_error::ConsoleCodegenError;
use margaret_container::container_error::ContainerError;
use margaret_generated_module::generated_module_error::GeneratedModuleError;
use margaret_http_codegen::http_codegen_error::HttpCodegenError;
use margaret_middleware_codegen::middleware_codegen_error::MiddlewareCodegenError;
use margaret_model_codegen::model_codegen_error::ModelCodegenError;
use margaret_request_binding_codegen::request_binding_error::RequestBindingError;
use margaret_service_codegen::service_codegen_error::ServiceCodegenError;
use margaret_views_codegen::views_codegen_error::ViewsCodegenError;
use margaret_websocket_codegen::websocket_codegen_error::WebSocketCodegenError;

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

    #[error("failed to read the console arguments: {source}")]
    ConsoleArgument {
        #[from]
        source: ConsoleArgumentCodegenError,
    },

    #[error(
        "a constructor injects the asset responder, but no esbuild metafile was found at the workspace root"
    )]
    AssetResponderWithoutMetafile,

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
        source: std::env::VarError,
    },

    #[error("no Cargo workspace root was found above the manifest directory '{start}'")]
    WorkspaceRootNotFound { start: PathBuf },

    #[error("failed to read the workspace manifest '{path}': {source}")]
    ReadWorkspaceManifest {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to parse the workspace manifest '{path}': {source}")]
    ParseWorkspaceManifest {
        path: PathBuf,
        source: Box<toml::de::Error>,
    },

    #[error("failed to create the generated directory '{path}': {source}")]
    CreateDirectory {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to read the generated directory '{path}': {source}")]
    ReadDirectory {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to read the esbuild metafile '{path}': {source}")]
    ReadMetafile {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to read the asset directory '{path}': {source}")]
    ReadAssetDirectory {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("the asset file name '{path}' is not valid UTF-8")]
    NonUtf8AssetPath { path: PathBuf },

    #[error(
        "the asset '{path}' is a symbolic link, which is not allowed under the asset directory"
    )]
    SymlinkAsset { path: PathBuf },

    #[error("the asset '{path}' is not a regular file")]
    NonRegularAsset { path: PathBuf },

    #[error("failed to write the generated source '{path}': {source}")]
    WriteSource {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("failed to remove the stale generated entry '{path}': {source}")]
    RemoveEntry {
        path: PathBuf,
        source: std::io::Error,
    },
}
