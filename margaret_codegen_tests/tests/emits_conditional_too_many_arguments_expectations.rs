use margaret_codegen_tests::generate_fixture::generate_fixture;
use margaret_codegen_tests::generated_module_source::generated_module_source;
use margaret_codegen_tests::generated_source::generated_source;

const EXPECT: &str = "#[expect(clippy::too_many_arguments";

fn collapsed(source: &str) -> String {
    source.split_whitespace().collect()
}

#[test]
fn eliminates_oversized_bootstrap_signatures_at_feature_boundaries() {
    let generated =
        generate_fixture("too_many_bootstrap_arguments").expect("the fixture generates");
    let http = generated_module_source(&generated, "http/server_public")
        .expect("the HTTP server module is generated");
    let views =
        generated_module_source(&generated, "views/build").expect("the views module is generated");
    let websocket = generated_module_source(&generated, "websocket")
        .expect("the WebSocket module is generated");
    let session = generated_module_source(&generated, "websocket/room")
        .expect("the WebSocket session module is generated");

    assert!(!collapsed(http).contains(EXPECT));
    assert!(!collapsed(views).contains(EXPECT));
    assert!(!collapsed(websocket).contains(EXPECT));
    assert!(!collapsed(session).contains(EXPECT));
}

#[test]
fn omits_expectations_when_bootstrap_functions_are_within_the_threshold() {
    let generated = generate_fixture("few_bootstrap_arguments").expect("the fixture generates");

    assert!(!collapsed(&generated_source(&generated)).contains(EXPECT));
}
