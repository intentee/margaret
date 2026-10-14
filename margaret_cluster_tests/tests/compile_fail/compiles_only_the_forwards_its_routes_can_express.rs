use trybuild::TestCases;

#[test]
fn compiles_only_the_forwards_its_routes_can_express() {
    let cases = TestCases::new();

    cases.pass("compile_pass/*.rs");
    cases.compile_fail("compile_fail/*.rs");
}
