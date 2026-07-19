use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeneratedModuleError {
    #[error("the generated tokens for module '{name}' do not form a valid Rust file: {source}")]
    InvalidGeneratedFile {
        name: String,
        #[source]
        source: syn::Error,
    },
}
