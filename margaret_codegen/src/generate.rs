use std::env;
use std::fs;
use std::path::Path;

use margaret_attributes::crate_root::CrateRoot;

use crate::build::build;

fn generate_into(manifest_directory: &Path) {
    let host_source = manifest_directory.join("src");
    let generated_directory = host_source.join("margaret");
    let crate_root = CrateRoot::new("crate", host_source);

    fs::create_dir_all(&generated_directory).expect("the generated directory is created");

    let umbrella = generated_directory.join("mod.rs");

    if !umbrella.exists() {
        fs::write(&umbrella, "").expect("the umbrella stub is written");
    }

    build(&crate_root)
        .expect("the crate is generated")
        .write_to(&generated_directory);

    println!(
        "cargo:rerun-if-changed={}",
        crate_root.source_directory.display()
    );
}

pub fn generate() {
    let manifest_directory =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set by cargo");

    generate_into(Path::new(&manifest_directory));
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use super::generate;
    use super::generate_into;

    const HOST_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create() -> Self {}
}
";

    fn write_crate(directory: &Path, lib_source: &str) {
        let source = directory.join("src");

        fs::create_dir_all(&source).expect("the src directory exists");
        fs::write(source.join("lib.rs"), lib_source).expect("lib.rs is written");
    }

    fn read_generated(manifest_directory: &Path, name: &str) -> String {
        fs::read_to_string(manifest_directory.join("src/margaret").join(name))
            .expect("the generated module exists")
    }

    #[test]
    fn generates_the_container_for_the_manifest_crate() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);

        generate_into(host.path());

        assert!(read_generated(host.path(), "container.rs").contains("struct Container"));
        assert!(read_generated(host.path(), "mod.rs").contains("pub mod container;"));
    }

    #[test]
    fn reuses_an_existing_umbrella_stub() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);

        generate_into(host.path());
        generate_into(host.path());

        assert!(read_generated(host.path(), "container.rs").contains("struct Container"));
    }

    #[test]
    fn reads_the_manifest_directory_from_the_environment() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);

        unsafe {
            env::set_var("CARGO_MANIFEST_DIR", host.path());
        }

        generate();

        assert!(read_generated(host.path(), "container.rs").contains("struct Container"));
    }
}
