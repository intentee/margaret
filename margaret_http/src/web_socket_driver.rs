use std::future::Future;
use std::pin::Pin;

pub type WebSocketDriver = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
