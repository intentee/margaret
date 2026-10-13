use thiserror::Error;

use crate::module_path::ModulePath;

#[derive(Debug, Error)]
pub enum GeneratedModuleError {
    #[error("the generated tokens for module '{path}' do not form a valid Rust file: {source}")]
    InvalidGeneratedFile {
        path: ModulePath,
        #[source]
        source: syn::Error,
    },
}
