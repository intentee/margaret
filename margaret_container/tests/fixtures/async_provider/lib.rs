trait Clock {}

#[provider(provides = Clock)]
struct ClockProvider;

impl ClockProvider {
    #[provide]
    async fn provide(&self) -> Arc<dyn Clock> {}
}
