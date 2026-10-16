use std::io;
use std::io::PipeWriter;

/// # Panics
///
/// Panics when the operating system cannot create a pipe.
#[must_use]
pub fn broken_pipe() -> PipeWriter {
    let (reader, writer) = io::pipe().expect("the pipe is created");

    drop(reader);

    writer
}
