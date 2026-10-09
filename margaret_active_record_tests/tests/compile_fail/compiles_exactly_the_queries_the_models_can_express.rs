use trybuild::TestCases;

#[test]
fn compiles_exactly_the_queries_the_models_can_express() {
    let cases = TestCases::new();

    cases.pass("compile_pass/*.rs");
    cases.compile_fail("compile_fail/*.rs");
}
