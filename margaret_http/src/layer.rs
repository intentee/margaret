use std::sync::Arc;

use crate::http_middleware::HttpMiddleware;
use crate::layerable_handler::LayerableHandler;

pub fn layer<TMiddleware, THandler>(
    middleware: Arc<TMiddleware>,
    inner: Arc<THandler>,
) -> Arc<THandler>
where
    TMiddleware: HttpMiddleware + Send + Sync + ?Sized + 'static,
    THandler: LayerableHandler + ?Sized,
{
    THandler::layered(middleware, inner)
}
