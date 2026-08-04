use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ScaffoldError {
    #[error("failed to create the scaffolded directory '{path}': {source}")]
    CreateDirectory {
        path: PathBuf,
        source: std::io::Error,
    },

    #[error("the scaffolded directory '{path}' already exists")]
    TargetDirectoryExists { path: PathBuf },

    #[error(
        "the workspace dependency '{name}' is neither a version string nor a table of dependency keys"
    )]
    WorkspaceDependencyMalformed { name: String },

    #[error("the workspace manifest declares no dependency named '{name}'")]
    WorkspaceDependencyMissing { name: String },

    #[error("the workspace manifest declares no [workspace.dependencies] table")]
    WorkspaceDependencyTableMissing,

    #[error("the workspace manifest is not valid TOML: {source}")]
    WorkspaceManifestMalformed {
        #[source]
        source: toml_edit::TomlError,
    },

    #[error("failed to write the scaffolded file '{path}': {source}")]
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },
}
