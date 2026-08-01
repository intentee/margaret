use thiserror::Error;

use margaret_route_parameter_binding::route_parameter_binding_error::RouteParameterBindingError;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("a consumer callback failed: {source:#}")]
    Consumer {
        #[source]
        source: anyhow::Error,
    },

    #[error(transparent)]
    RequestBinding(#[from] RouteParameterBindingError),

    #[error("the responder forward cycle re-entered '{responder}'")]
    ForwardCycle { responder: &'static str },

    #[error("no forward target is registered for '{responder}' on this server")]
    UnknownForwardTarget { responder: &'static str },
}

impl HandlerError {
    #[must_use]
    pub fn consumer(source: anyhow::Error) -> Self {
        Self::Consumer { source }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::HandlerError;

    #[test]
    fn preserves_a_consumer_error_as_its_source() {
        let error = HandlerError::consumer(anyhow::anyhow!("database unavailable"));

        assert_eq!(
            error
                .source()
                .expect("the consumer error is preserved")
                .to_string(),
            "database unavailable"
        );
    }
}
