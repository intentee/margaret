use std::path::PathBuf;

use margaret_spiffe_svid_manager_tests::run_spire_command::run_spire_command;
use margaret_spiffe_svid_manager_tests::scenario_fixture_paths::spire_server_exits_non_zero;

#[tokio::test]
async fn exits_non_zero_fixture_surfaces_as_command_error() {
    let bogus_socket = PathBuf::from("/tmp/irrelevant-coverage-test.sock");

    let result = run_spire_command(&spire_server_exits_non_zero(), &bogus_socket, &["any"]).await;

    assert!(result.is_err());
}
