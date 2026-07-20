#[derive(Debug, thiserror::Error)]
pub enum EsbuildMetafileCodegenError {
    #[error("failed to parse the esbuild metafile")]
    InvalidMetafile(#[from] serde_json::Error),
}
