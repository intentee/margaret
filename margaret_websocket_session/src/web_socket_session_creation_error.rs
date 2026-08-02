use thiserror::Error;

use margaret_route_parameter_binding::route_parameter_binding_error::RouteParameterBindingError;

#[derive(Debug, Error)]
pub enum WebSocketSessionCreationError {
    #[error("a consumer callback failed while creating a websocket session: {source:#}")]
    Consumer {
        #[source]
        source: anyhow::Error,
    },

    #[error(transparent)]
    RouteParameterBinding(#[from] RouteParameterBindingError),
}

impl WebSocketSessionCreationError {
    #[must_use]
    pub fn consumer(source: anyhow::Error) -> Self {
        Self::Consumer { source }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::WebSocketSessionCreationError;

    #[test]
    fn preserves_a_consumer_error_as_its_source() {
        let error =
            WebSocketSessionCreationError::consumer(anyhow::anyhow!("database unavailable"));

        assert_eq!(
            error
                .source()
                .expect("the consumer error is preserved")
                .to_string(),
            "database unavailable"
        );
    }
}
