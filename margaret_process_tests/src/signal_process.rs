use std::process::Command;
use std::process::ExitStatus;

use crate::process_signal::ProcessSignal;

/// # Panics
///
/// Panics when the `kill` utility cannot be run.
#[must_use]
pub fn signal_process(pid: u32, signal: ProcessSignal) -> ExitStatus {
    Command::new("kill")
        .arg("-s")
        .arg(signal.name())
        .arg(pid.to_string())
        .status()
        .expect("the kill utility runs")
}
