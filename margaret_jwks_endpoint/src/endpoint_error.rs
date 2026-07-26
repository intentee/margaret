use thiserror::Error;

#[derive(Debug, Error)]
pub enum EndpointError {
    #[error("a user-implemented jwks endpoint provider returned an error: {0:#}")]
    UserError(anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::EndpointError;

    #[test]
    fn renders_the_wrapped_source() {
        let error = EndpointError::UserError(anyhow::anyhow!("the srv record has no targets"));

        assert!(error.to_string().contains("the srv record has no targets"));
    }
}
