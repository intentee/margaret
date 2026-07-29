use std::path::PathBuf;

use margaret_attributes::crate_root::CrateRoot;
use margaret_codegen::build::build;
use margaret_codegen::generated_code::GeneratedCode;
use margaret_codegen_tests::generated_module_source::generated_module_source;

fn generated() -> GeneratedCode {
    let source_directory =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/fallible_roles/src");
    let assets_directory = source_directory.join("assets");

    build(
        &CrateRoot::new("crate", source_directory),
        None,
        &assets_directory,
        ".",
    )
    .expect("the fallible_roles fixture generates")
}

fn collapsed(source: &str) -> String {
    source.split_whitespace().collect()
}

fn module(generated: &GeneratedCode, name: &str) -> String {
    collapsed(generated_module_source(generated, name).expect("the module is generated"))
}

#[test]
fn reports_a_fallible_service_registration_into_the_command_outcome() {
    assert!(module(&generated(), "serve").contains("inner:container.worker_worker()"));
}

#[test]
fn reports_a_fallible_console_command_into_the_command_outcome() {
    let source = module(&generated(), "run");

    assert!(source.contains(
        "matchsuper::container::build::construct_boot_boot(){Ok(value)=>value,Err(error)=>"
    ));
    assert!(source.contains("margaret::framework::console::report_failure::report_failure(error"));
    assert!(source.contains(
        "margaret::framework::console::command_outcome::CommandOutcome::from_user_result("
    ));
}

#[test]
fn views_builder_reads_only_completed_dependencies() {
    let source = module(&generated(), "views/build");

    assert!(source.contains("->super::Views"));
    assert!(source.contains("container.home_view_home_view()"));
    assert!(!source.contains("ConstructionError"));
}

#[test]
fn http_server_assembly_reads_only_completed_dependencies() {
    let source = module(&generated(), "http/server_public");

    assert!(source.contains("container.get_home_get_home()"));
    assert!(source.contains("super::super::websocket::public_routes(container,routes)"));
    assert!(source.contains(
        "->::std::result::Result<margaret::framework::http::server_routes::ServerRoutes,margaret::framework::http::matchit::InsertError,>"
    ));
    assert!(!source.contains("ConstructionError"));
}

#[test]
fn websocket_dispatch_reads_only_completed_dependencies() {
    let source = module(
        &generated(),
        "websocket/chat_chat_session_chat_session/upgrade_entry",
    );

    assert!(source.contains("handler:container.chat_chat_responder_chat_responder()"));
    assert!(source.contains("dispatch_table(container)"));
    assert!(!source.contains("ConstructionError"));
}
