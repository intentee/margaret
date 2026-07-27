#[test]
fn user_result_rejects_non_anyhow_errors_and_non_command_outcomes() {
    let cases = trybuild::TestCases::new();

    cases.compile_fail("compile_fail/*.rs");
}
