use std::path::PathBuf;

use syn::File;

pub(crate) struct ScaffoldedModule {
    pub file: File,
    pub relative_path: PathBuf,
}
