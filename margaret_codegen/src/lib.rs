pub mod codegen_error;

use std::fs;
use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_container::generated_source::GeneratedSource;

use crate::codegen_error::CodegenError;

pub fn build(manifest_directory: impl AsRef<Path>) -> Result<(), CodegenError> {
    let source_directory = manifest_directory.as_ref().join("src");
    let generated_directory = source_directory.join("margaret");
    let umbrella_path = source_directory.join("margaret.rs");
    let container_path = generated_directory.join("container.rs");
    let http_path = generated_directory.join("http.rs");
    let console_path = generated_directory.join("console.rs");

    fs::create_dir_all(&generated_directory).expect("the generated directory is created");

    if !container_path.exists() {
        fs::write(&container_path, "").expect("the container stub is written");
    }

    if !umbrella_path.exists() {
        fs::write(&umbrella_path, render_umbrella(false, false))
            .expect("the umbrella stub is written");
    }

    let index = AttributeIndex::from_crate_root("crate", &source_directory)?;
    let input_selectors = margaret_console_codegen::input_selectors();

    margaret_container::render_container(&index, &input_selectors)?
        .write_if_changed(&container_path)
        .expect("the generated container is written");

    let has_http = margaret_http_codegen::has_responders(&index);

    if has_http {
        GeneratedSource::new(margaret_http_codegen::render_http(&index)?)
            .write_if_changed(&http_path)
            .expect("the generated http source is written");
    } else if http_path.exists() {
        fs::remove_file(&http_path).expect("the stale http source is removed");
    }

    let has_console = margaret_console_codegen::has_commands(&index) || has_http;

    if has_console {
        GeneratedSource::new(margaret_console_codegen::render_console(&index, has_http)?)
            .write_if_changed(&console_path)
            .expect("the generated console source is written");
    } else if console_path.exists() {
        fs::remove_file(&console_path).expect("the stale console source is removed");
    }

    GeneratedSource::new(render_umbrella(has_http, has_console))
        .write_if_changed(&umbrella_path)
        .expect("the umbrella module is written");

    println!("cargo:rerun-if-changed={}", source_directory.display());

    Ok(())
}

fn render_umbrella(has_http: bool, has_console: bool) -> String {
    let mut umbrella = String::from("#[rustfmt::skip]\npub mod container;\n");

    if has_http {
        umbrella.push_str("#[rustfmt::skip]\npub mod http;\n");
    }

    if has_console {
        umbrella.push_str("#[rustfmt::skip]\npub mod console;\n");
    }

    umbrella
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use crate::build;

    const WEB_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[responds_to_http(method = Get, path = \"/x\")]
struct Page;

impl Page {
    #[constructor]
    fn create() -> Self {}
}
";

    const PLAIN_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create() -> Self {}
}
";

    const COMMAND_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
#[console_command(name = \"greet\")]
struct Greet;

impl Greet {
    #[constructor]
    fn create() -> Self {}
}
";

    fn write_lib(directory: &TempDir, lib_source: &str) {
        let source_directory = directory.path().join("src");

        fs::create_dir_all(&source_directory).expect("the src directory exists");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");
    }

    fn crate_with(lib_source: &str) -> TempDir {
        let directory = tempdir().expect("a temporary crate directory is created");

        write_lib(&directory, lib_source);

        directory
    }

    fn read(directory: &TempDir, relative: &str) -> String {
        fs::read_to_string(directory.path().join(relative)).expect("the generated file exists")
    }

    #[test]
    fn generates_container_and_server_when_responders_exist() {
        let directory = crate_with(WEB_CRATE);

        build(directory.path()).expect("the build succeeds");

        let umbrella = read(&directory, "src/margaret.rs");
        let http = read(&directory, "src/margaret/http.rs");

        assert!(umbrella.contains("pub mod container;"));
        assert!(umbrella.contains("pub mod http;"));
        assert!(umbrella.contains("pub mod console;"));
        assert!(http.contains("use super::container::Container"));
        assert!(http.contains("fn server"));
        assert!(read(&directory, "src/margaret/container.rs").contains("struct Container"));
        assert!(read(&directory, "src/margaret/console.rs").contains("\"serve\""));
    }

    #[test]
    fn generates_a_console_for_commands_without_http() {
        let directory = crate_with(COMMAND_CRATE);

        build(directory.path()).expect("the build succeeds");

        let umbrella = read(&directory, "src/margaret.rs");

        assert!(umbrella.contains("pub mod console;"));
        assert!(!umbrella.contains("pub mod http;"));
        assert!(!directory.path().join("src/margaret/http.rs").exists());
        assert!(read(&directory, "src/margaret/console.rs").contains("\"greet\""));
    }

    #[test]
    fn generates_only_container_without_responders() {
        let directory = crate_with(PLAIN_CRATE);

        build(directory.path()).expect("the build succeeds");

        let umbrella = read(&directory, "src/margaret.rs");

        assert!(umbrella.contains("pub mod container;"));
        assert!(!umbrella.contains("pub mod http;"));
        assert!(!umbrella.contains("pub mod console;"));
        assert!(!directory.path().join("src/margaret/http.rs").exists());
        assert!(!directory.path().join("src/margaret/console.rs").exists());
        assert!(read(&directory, "src/margaret/container.rs").contains("struct Container"));
    }

    #[test]
    fn removes_the_server_when_responders_are_removed() {
        let directory = crate_with(WEB_CRATE);

        build(directory.path()).expect("the first build succeeds");

        assert!(directory.path().join("src/margaret/http.rs").exists());

        write_lib(&directory, PLAIN_CRATE);

        build(directory.path()).expect("the second build succeeds");

        assert!(!directory.path().join("src/margaret/http.rs").exists());
        assert!(!directory.path().join("src/margaret/console.rs").exists());
        assert!(!read(&directory, "src/margaret.rs").contains("pub mod http;"));
        assert!(!read(&directory, "src/margaret.rs").contains("pub mod console;"));
    }

    #[test]
    fn is_idempotent() {
        let directory = crate_with(WEB_CRATE);

        build(directory.path()).expect("the first build succeeds");

        let umbrella = read(&directory, "src/margaret.rs");
        let http = read(&directory, "src/margaret/http.rs");
        let console = read(&directory, "src/margaret/console.rs");
        let container = read(&directory, "src/margaret/container.rs");

        build(directory.path()).expect("the second build succeeds");

        assert_eq!(read(&directory, "src/margaret.rs"), umbrella);
        assert_eq!(read(&directory, "src/margaret/http.rs"), http);
        assert_eq!(read(&directory, "src/margaret/console.rs"), console);
        assert_eq!(read(&directory, "src/margaret/container.rs"), container);
    }

    #[test]
    fn propagates_an_index_failure() {
        let directory = crate_with("use other::*;\n");

        let message = build(directory.path())
            .expect_err("the build fails")
            .to_string();

        assert!(message.contains("failed to index"));
    }

    #[test]
    fn propagates_a_container_failure() {
        let directory =
            crate_with("#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nenum Bad {}\n");

        let message = build(directory.path())
            .expect_err("the build fails")
            .to_string();

        assert!(message.contains("failed to generate the dependency container"));
    }

    #[test]
    fn propagates_an_http_failure() {
        let directory = crate_with(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\n#[responds_to_http(path = \"/x\")]\nstruct Bad;\n\nimpl Bad {\n    #[constructor]\n    fn create() -> Self {}\n}\n",
        );

        let message = build(directory.path())
            .expect_err("the build fails")
            .to_string();

        assert!(message.contains("missing the 'method'"));
    }

    #[test]
    fn propagates_a_console_failure() {
        let directory = crate_with(
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nstruct Config;\n\nimpl Config {\n    #[constructor]\n    fn create() -> Self {}\n}\n\n#[console_command(name = \"bad\")]\nenum Bad {}\n",
        );

        let message = build(directory.path())
            .expect_err("the build fails")
            .to_string();

        assert!(message.contains("failed to generate the console"));
    }
}
