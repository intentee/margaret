use std::future::Future;
use std::pin::Pin;

pub type ConnectionDriver = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
