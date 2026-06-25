use std::future::Future;

pub trait RouteParameterBinder {
    type Model;

    fn bind(&self, value: String) -> impl Future<Output = Option<Self::Model>> + Send;
}
