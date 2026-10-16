use std::fmt::Display;
use std::io;
use std::io::Write;

pub(crate) fn write_standard_output(output: impl Display) -> io::Result<()> {
    writeln!(io::stdout().lock(), "{output}")
}
