use std::path::PathBuf;

use margaret_attributes::crate_root::CrateRoot;
use margaret_codegen::build::build;
use margaret_codegen::generated_code::GeneratedCode;
use margaret_codegen_tests::generated_module_source::generated_module_source;

fn collapsed(source: &str) -> String {
    source.split_whitespace().collect()
}

fn generated() -> GeneratedCode {
    let source_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../margaret_codegen_reusable_websocket_protocol_fixture/src");
    let assets_directory = source_directory.join("assets");

    build(
        &CrateRoot::new("crate", source_directory),
        None,
        &assets_directory,
        ".",
    )
    .expect("the reusable WebSocket protocol fixture generates")
}

fn module(generated: &GeneratedCode, name: &str) -> String {
    collapsed(generated_module_source(generated, name).expect("the module is generated"))
}

#[test]
fn generates_a_dispatch_for_each_session_that_reuses_a_message() {
    let generated = generated();
    let alpha = module(&generated, "websocket/alpha_session_alpha_session");
    let beta = module(&generated, "websocket/beta_session_beta_session");
    let websocket = module(&generated, "websocket");

    assert!(alpha.contains("crate::alpha_session::on_conversation_message::OnConversationMessage"));
    assert!(beta.contains("crate::beta_session::on_conversation_message::OnConversationMessage"));
    assert_eq!(
        alpha
            .matches("\"conversation_message\".to_string()")
            .count(),
        1
    );
    assert_eq!(
        beta.matches("\"conversation_message\".to_string()").count(),
        1
    );
    assert_eq!(
        websocket
            .matches("WebSocketRequestMessageforcrate::conversation_message::ConversationMessage",)
            .count(),
        1
    );
}
