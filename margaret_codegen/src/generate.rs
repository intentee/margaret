use std::env;
use std::fs;
use std::path::Path;

use margaret_attributes::crate_root::CrateRoot;

use crate::build::build;

fn generate_into(manifest_directory: &Path, extra_crate_directories: &[&str]) {
    let host_source = manifest_directory.join("src");
    let generated_directory = host_source.join("margaret");

    let mut crates = vec![CrateRoot::new("crate", host_source)];

    for extra_crate_directory in extra_crate_directories {
        let crate_directory = manifest_directory.join(extra_crate_directory);
        let crate_name = read_package_name(&crate_directory.join("Cargo.toml"));

        crates.push(CrateRoot::new(crate_name, crate_directory.join("src")));
    }

    fs::create_dir_all(&generated_directory).expect("the generated directory is created");

    let umbrella = generated_directory.join("mod.rs");

    if !umbrella.exists() {
        fs::write(&umbrella, "").expect("the umbrella stub is written");
    }

    build(&crates)
        .expect("the crate is generated")
        .write_to(&generated_directory);

    for crate_root in &crates {
        println!(
            "cargo:rerun-if-changed={}",
            crate_root.source_directory.display()
        );
    }
}

fn read_package_name(cargo_toml: &Path) -> String {
    let contents = fs::read_to_string(cargo_toml).expect("the crate's Cargo.toml is readable");
    let manifest: toml::Table = toml::from_str(&contents).expect("the crate's Cargo.toml parses");

    manifest
        .get("package")
        .and_then(|package| package.get("name"))
        .and_then(|name| name.as_str())
        .expect("the crate's Cargo.toml declares a package name")
        .to_string()
}

pub fn generate(extra_crate_directories: &[&str]) {
    let manifest_directory =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set by cargo");

    generate_into(Path::new(&manifest_directory), extra_crate_directories);
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

        generate_into(host.path(), &[]);

        assert!(read_generated(host.path(), "container.rs").contains("struct Container"));
        assert!(read_generated(host.path(), "mod.rs").contains("pub mod container;"));
    }

    #[test]
    fn reads_extra_crate_names_from_their_manifests() {
        let host = tempdir().expect("a host crate directory");
        write_crate(
            host.path(),
            "#[rustfmt::skip]\npub mod margaret;\n\n#[singleton]\nstruct App;\n\nimpl App {\n    #[constructor]\n    fn create(metrics: Arc<myplugin::Metrics>) -> Self {}\n}\n",
        );

        let plugin = host.path().join("plugin");
        fs::create_dir_all(&plugin).expect("the plugin directory exists");
        fs::write(
            plugin.join("Cargo.toml"),
            "[package]\nname = \"myplugin\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .expect("the plugin manifest is written");
        write_crate(
            &plugin,
            "#[singleton]\nstruct Metrics;\n\nimpl Metrics {\n    #[constructor]\n    fn create() -> Self {}\n}\n",
        );

        generate_into(host.path(), &["plugin"]);

        assert!(read_generated(host.path(), "container.rs").contains("myplugin::Metrics"));
        assert!(read_generated(host.path(), "container.rs").contains("crate::App"));
    }

    #[test]
    fn reuses_an_existing_umbrella_stub() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);

        generate_into(host.path(), &[]);
        generate_into(host.path(), &[]);

        assert!(read_generated(host.path(), "container.rs").contains("struct Container"));
    }

    #[test]
    fn reads_the_manifest_directory_from_the_environment() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);

        unsafe {
            env::set_var("CARGO_MANIFEST_DIR", host.path());
        }

        generate(&[]);

        assert!(read_generated(host.path(), "container.rs").contains("struct Container"));
    }
}
