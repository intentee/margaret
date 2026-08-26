#[singleton]
struct First;

impl First {
    #[constructor]
    fn create(#[environment_variable(from = "UPLOAD_ROOT")] upload_root: String) -> anyhow::Result<Self> {}
}

#[singleton]
struct Second;

impl Second {
    #[constructor]
    fn create(#[environment_variable(from = "UPLOAD_ROOT")] upload_root: std::path::PathBuf) -> anyhow::Result<Self> {}
}
