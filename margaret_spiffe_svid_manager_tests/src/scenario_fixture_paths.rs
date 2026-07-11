use std::path::PathBuf;

#[must_use]
fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}

#[must_use]
pub fn spire_server_exits_immediately() -> PathBuf {
    fixtures_dir().join("spire_server_exits_immediately.sh")
}

#[must_use]
pub fn spire_server_exits_non_zero() -> PathBuf {
    fixtures_dir().join("spire_server_exits_non_zero.sh")
}

#[must_use]
pub fn spire_server_outputs_unparseable_join_token() -> PathBuf {
    fixtures_dir().join("spire_server_outputs_unparseable_join_token.sh")
}

#[must_use]
pub fn spire_server_real_daemon_with_failing_bundle_cli() -> PathBuf {
    fixtures_dir().join("spire_server_real_daemon_with_failing_bundle_cli.sh")
}

#[must_use]
pub fn spire_server_real_daemon_with_failing_entry_cli() -> PathBuf {
    fixtures_dir().join("spire_server_real_daemon_with_failing_entry_cli.sh")
}

#[must_use]
pub fn spire_server_real_daemon_with_unparseable_token_cli() -> PathBuf {
    fixtures_dir().join("spire_server_real_daemon_with_unparseable_token_cli.sh")
}
