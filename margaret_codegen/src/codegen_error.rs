use margaret_attributes::attribute_error::AttributeError;
use margaret_container::container_error::ContainerError;
use margaret_http_codegen::http_codegen_error::HttpCodegenError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodegenError {
    #[error("failed to index the crate: {source}")]
    Index {
        #[from]
        source: AttributeError,
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
}
