use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("a user-implemented authenticated user provider returned an error: {0:#}")]
    UserError(anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::IdentityError;

    #[test]
    fn renders_the_wrapped_source() {
        let error = IdentityError::UserError(anyhow::anyhow!("the session store is unavailable"));

        assert!(error.to_string().contains("the session store is unavailable"));
    }
}
