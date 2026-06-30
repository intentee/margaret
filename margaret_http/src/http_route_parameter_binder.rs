use async_trait::async_trait;

#[async_trait]
pub trait HttpRouteParameterBinder {
    type Model;

    async fn bind(&self, value: String) -> Option<Self::Model>;
}
