#[derive(Debug, thiserror::Error)]
pub enum AssetBagCodegenError {
    #[error("the esbuild input `{input}` is both an entry point and a static asset")]
    AmbiguousInput { input: String },
    #[error(
        "the static esbuild input `{input}` resolves to {output_count} outputs instead of exactly one"
    )]
    AmbiguousStaticInput { input: String, output_count: usize },
    #[error("two esbuild outputs declare the same entry point `{input}`")]
    DuplicateEntrypoint { input: String },
    #[error("the esbuild metafile declares no outputs")]
    EmptyMetafile,
    #[error("the entry-point output `{output}` is not present in the metafile")]
    EntrypointOutputMissing { output: String },
    #[error("the esbuild outputs span multiple directories `{first}` and `{second}`")]
    InconsistentOutputDirectory { first: String, second: String },
    #[error("the esbuild input `{input}` is not present in the metafile")]
    InputNotInMetafile { input: String },
    #[error("failed to parse the esbuild metafile")]
    MetafileParse(#[from] serde_json::Error),
    #[error("the esbuild output `{output}` is not inside an output directory")]
    OutputMissingDirectory { output: String },
    #[error("the entry-point output `{output}` is neither a script nor a stylesheet")]
    UnsupportedIncludeOutput { output: String },
}
