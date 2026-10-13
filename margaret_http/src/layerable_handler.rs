use std::sync::Arc;

use crate::content_handler::ContentHandler;
use crate::head_handler::HeadHandler;
use crate::http_middleware::HttpMiddleware;
use crate::layered_content_handler::LayeredContentHandler;
use crate::layered_head_handler::LayeredHeadHandler;

pub trait LayerableHandler: Send + Sync {
    fn layered<TMiddleware>(middleware: Arc<TMiddleware>, inner: Arc<Self>) -> Arc<Self>
    where
        TMiddleware: HttpMiddleware + Send + Sync + ?Sized + 'static;
}

impl LayerableHandler for dyn ContentHandler {
    fn layered<TMiddleware>(middleware: Arc<TMiddleware>, inner: Arc<Self>) -> Arc<Self>
    where
        TMiddleware: HttpMiddleware + Send + Sync + ?Sized + 'static,
    {
        Arc::new(LayeredContentHandler { inner, middleware })
    }
}

impl LayerableHandler for dyn HeadHandler {
    fn layered<TMiddleware>(middleware: Arc<TMiddleware>, inner: Arc<Self>) -> Arc<Self>
    where
        TMiddleware: HttpMiddleware + Send + Sync + ?Sized + 'static,
    {
        Arc::new(LayeredHeadHandler { inner, middleware })
    }
}
