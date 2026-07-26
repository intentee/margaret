use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestBindingError {
    #[error("the binder for route parameter '{parameter}' failed: {source:#}")]
    RouteParameterBinder {
        parameter: &'static str,
        #[source]
        source: anyhow::Error,
    },
}
