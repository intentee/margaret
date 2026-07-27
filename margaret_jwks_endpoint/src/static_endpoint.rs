use async_trait::async_trait;
use url::Url;

use crate::provides_endpoint::ProvidesEndpoint;

pub struct StaticEndpoint {
    url: Url,
}

impl StaticEndpoint {
    #[must_use]
    pub fn new(url: Url) -> Self {
        Self { url }
    }
}

#[async_trait]
impl ProvidesEndpoint for StaticEndpoint {
    async fn provide(&self) -> anyhow::Result<Url> {
        Ok(self.url.clone())
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::provides_endpoint::ProvidesEndpoint;
    use crate::static_endpoint::StaticEndpoint;

    #[tokio::test]
    async fn provides_the_configured_url() {
        let url = Url::parse("https://issuer.example.org").expect("the url parses");
        let endpoint = StaticEndpoint::new(url.clone());

        assert_eq!(
            endpoint.provide().await.expect("the endpoint resolves"),
            url
        );
    }
}
