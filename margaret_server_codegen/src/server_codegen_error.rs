use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerCodegenError {
    #[error(
        "the server name '{name}' must be a snake_case identifier usable as a `routes` accessor"
    )]
    ServerNameNotSnakeCase { name: String },
}
