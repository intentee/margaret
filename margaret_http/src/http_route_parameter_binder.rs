use async_trait::async_trait;

#[async_trait]
pub trait HttpRouteParameterBinder {
    type Model;
    type Error: std::fmt::Display;

    async fn bind(&self, value: String) -> Result<Option<Self::Model>, Self::Error>;
}
