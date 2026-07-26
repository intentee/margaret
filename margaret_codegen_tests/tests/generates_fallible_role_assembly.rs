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
    assert!(module(&generated(), "serve").contains(
        "inner:(matchcontainer.worker_worker().await{Ok(value)=>value,Err(error)=>{returnmargaret::framework::console::report_failure::report_failure(error,);}})"
    ));
}

#[test]
fn reports_a_fallible_console_command_into_the_command_outcome() {
    assert!(module(&generated(), "run").contains(
        "Some((\"boot\",_matches))=>{match(matchcontainer.boot_boot().await{Ok(value)=>value,Err(error)=>{returnmargaret::framework::console::report_failure::report_failure(error,);}}).run().await{Ok(())=>{margaret::framework::console::command_outcome::CommandOutcome::Succeeded}Err(error)=>{margaret::framework::console::report_failure::report_failure(margaret::framework::console::console_error::ConsoleError::UserError(error,),)}}}"
    ));
}

#[test]
fn threads_construction_errors_through_the_views_builder() {
    assert!(module(&generated(), "views/build").contains(
        "->::std::result::Result<super::Views,std::sync::Arc<margaret::framework::container_error::construction_error::ConstructionError,>,>"
    ));
}

#[test]
fn threads_construction_errors_through_the_http_server_assembly() {
    let source = module(&generated(), "http/server_public");

    assert!(source.contains("container.get_home_get_home().await?"));
    assert!(source.contains("super::super::websocket::public_routes(container,routes).await?"));
    assert!(source.contains(
        "->::std::result::Result<::std::result::Result<margaret::framework::http::server_routes::ServerRoutes,margaret::framework::http::matchit::InsertError,>,std::sync::Arc<margaret::framework::container_error::construction_error::ConstructionError,>,>"
    ));
}

#[test]
fn threads_construction_errors_through_the_websocket_dispatch() {
    let source = module(&generated(), "websocket/chat_chat_session");

    assert!(source.contains("handler:container.chat_chat_responder().await?"));
    assert!(source.contains("dispatch_table(container).await?"));
}
