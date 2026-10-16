use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ConsoleOutputError {
    #[error("failed to write the command output to standard output: {source}")]
    CommandOutput { source: io::Error },

    #[error("failed to write the command help to standard output: {source}")]
    Help { source: io::Error },
}
