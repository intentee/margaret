use std::sync::Arc;

#[singleton]
struct DnsResolver;

impl DnsResolver {
    #[constructor]
    fn new() -> anyhow::Result<Self> {}
}

#[singleton]
struct Catalog;

impl Catalog {
    #[constructor]
    fn new(
        resolver: Arc<DnsResolver>,
        #[console_argument(from = "issuer")] issuer: String,
    ) -> anyhow::Result<Self> {}
}
