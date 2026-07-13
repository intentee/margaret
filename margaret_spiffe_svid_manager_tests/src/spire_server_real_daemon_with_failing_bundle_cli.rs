use std::path::PathBuf;

use crate::fixtures_dir::fixtures_dir;

#[must_use]
pub fn spire_server_real_daemon_with_failing_bundle_cli() -> PathBuf {
    fixtures_dir().join("spire_server_real_daemon_with_failing_bundle_cli.sh")
}
