use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConsoleError {
    #[error("a user-implemented console command returned an error: {0:#}")]
    UserError(anyhow::Error),
}

#[cfg(test)]
mod tests {
    use super::ConsoleError;

    #[test]
    fn renders_the_wrapped_source() {
        let error = ConsoleError::UserError(anyhow::anyhow!("the migration could not be applied"));

        assert!(error.to_string().contains("the migration could not be applied"));
    }
}
