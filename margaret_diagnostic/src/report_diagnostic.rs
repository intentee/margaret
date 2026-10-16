use std::fmt::Display;
use std::io;
use std::io::Write;

pub fn report_diagnostic(diagnostic: impl Display) {
    match writeln!(io::stderr(), "{diagnostic}") {
        Ok(()) | Err(_) => {}
    }
}
