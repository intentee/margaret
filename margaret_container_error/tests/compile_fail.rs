#[test]
fn wrap_rejects_outcomes_outside_the_anyhow_result_contract() {
    let cases = trybuild::TestCases::new();

    cases.compile_fail("compile_fail/*.rs");
}
