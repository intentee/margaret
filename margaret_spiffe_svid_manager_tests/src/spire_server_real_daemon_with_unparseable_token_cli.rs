use std::path::PathBuf;

use crate::fixtures_dir::fixtures_dir;

#[must_use]
pub fn spire_server_real_daemon_with_unparseable_token_cli() -> PathBuf {
    fixtures_dir().join("spire_server_real_daemon_with_unparseable_token_cli.sh")
}
