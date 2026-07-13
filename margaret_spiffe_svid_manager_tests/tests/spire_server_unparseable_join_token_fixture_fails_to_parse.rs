use std::path::PathBuf;

use margaret_spiffe_svid_manager_tests::parse_join_token::parse_join_token;
use margaret_spiffe_svid_manager_tests::run_spire_command::run_spire_command;
use margaret_spiffe_svid_manager_tests::spire_server_outputs_unparseable_join_token::spire_server_outputs_unparseable_join_token;

#[tokio::test]
async fn unparseable_join_token_fixture_output_fails_to_parse() {
    let bogus_socket = PathBuf::from("/tmp/irrelevant-coverage-test.sock");

    let output = run_spire_command(
        &spire_server_outputs_unparseable_join_token(),
        &bogus_socket,
        &["token", "generate"],
    )
    .await
    .unwrap();

    let parsed = parse_join_token(&output);

    assert!(parsed.is_err());
}
