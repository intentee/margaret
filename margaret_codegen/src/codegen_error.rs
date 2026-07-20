use std::path::PathBuf;

use thiserror::Error;

use margaret_asset_bag_codegen::asset_bag_codegen_error::AssetBagCodegenError;
use margaret_attributes::attribute_error::AttributeError;
use margaret_console_codegen::console_codegen_error::ConsoleCodegenError;
use margaret_container::container_error::ContainerError;
use margaret_generated_module::generated_module_error::GeneratedModuleError;
use margaret_http_codegen::http_codegen_error::HttpCodegenError;
use margaret_model_codegen::model_codegen_error::ModelCodegenError;
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
