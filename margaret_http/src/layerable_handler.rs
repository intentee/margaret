use std::sync::Arc;

use crate::content_handler::ContentHandler;
use crate::head_handler::HeadHandler;
use crate::http_middleware::HttpMiddleware;
use crate::layered_content_handler::LayeredContentHandler;
use crate::layered_head_handler::LayeredHeadHandler;

pub trait LayerableHandler: Send + Sync {
    type Target;

    fn layered<TMiddleware>(middleware: Arc<TMiddleware>, inner: Arc<Self>) -> Arc<Self>
    where
        TMiddleware: HttpMiddleware<Self::Target> + Send + Sync + ?Sized + 'static;
}

impl<TTarget: Send + 'static> LayerableHandler for dyn ContentHandler<TTarget> {
    type Target = TTarget;

    fn layered<TMiddleware>(middleware: Arc<TMiddleware>, inner: Arc<Self>) -> Arc<Self>
    where
        TMiddleware: HttpMiddleware<TTarget> + Send + Sync + ?Sized + 'static,
    {
        Arc::new(LayeredContentHandler { inner, middleware })
    }
}

impl<TTarget: Send + 'static> LayerableHandler for dyn HeadHandler<TTarget> {
    type Target = TTarget;

    fn layered<TMiddleware>(middleware: Arc<TMiddleware>, inner: Arc<Self>) -> Arc<Self>
    where
        TMiddleware: HttpMiddleware<TTarget> + Send + Sync + ?Sized + 'static,
    {
        Arc::new(LayeredHeadHandler { inner, middleware })
    }
}
