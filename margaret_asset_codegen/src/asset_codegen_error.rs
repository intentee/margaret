#[derive(Debug, thiserror::Error)]
pub enum AssetCodegenError {
    #[error("failed to parse the esbuild metafile")]
    InvalidMetafile(#[from] serde_json::Error),
}
