#[singleton]
struct Consumer;

impl Consumer {
    #[constructor]
    fn new(
        #[environment_variable(from = "SIGNING_STORE")]
        #[jwks_secret_store(server)]
        store: String,
    ) -> anyhow::Result<Self> {
    }
}
