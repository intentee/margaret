use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use margaret_attributes::crate_root::CrateRoot;

use crate::assets_directory_name::ASSETS_DIRECTORY_NAME;
use crate::build::build;
use crate::codegen_error::CodegenError;
use crate::umbrella_module_name::UMBRELLA_MODULE_NAME;
use crate::workspace_root::workspace_root;

fn read_metafile(metafile_path: &Path) -> Result<Option<String>, CodegenError> {
    match fs::read_to_string(metafile_path) {
        Ok(contents) => Ok(Some(contents)),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(None),
        Err(source) => Err(CodegenError::ReadMetafile {
            path: metafile_path.to_path_buf(),
            source,
        }),
    }
}

fn generate_into(manifest_directory: &Path) -> Result<(), CodegenError> {
    let host_source = manifest_directory.join("src");
    let generated_directory = manifest_directory.join(UMBRELLA_MODULE_NAME);
    let location = workspace_root(manifest_directory)?;
    let metafile_path = location.root().join("esbuild-meta.json");
    let assets_directory = location.root().join(ASSETS_DIRECTORY_NAME);
    let crate_root = CrateRoot::new("crate", host_source);

    fs::create_dir_all(&generated_directory).map_err(|source| CodegenError::CreateDirectory {
        path: generated_directory.clone(),
        source,
    })?;

    let metafile_contents = read_metafile(&metafile_path)?;

    build(
        &crate_root,
        metafile_contents.as_deref(),
        &assets_directory,
        location.embed_relative(),
    )?
    .write_to(&generated_directory)?;

    println!(
        "cargo:rerun-if-changed={}",
        crate_root.source_directory.display()
    );
    println!("cargo:rerun-if-changed={}", metafile_path.display());
    println!("cargo:rerun-if-changed={}", assets_directory.display());

    Ok(())
}

/// # Errors
///
/// Returns `CodegenError::ManifestDirectory`.
pub fn generate() -> Result<(), CodegenError> {
    let manifest_directory = env::var("CARGO_MANIFEST_DIR")
        .map_err(|source| CodegenError::ManifestDirectory { source })?;

    generate_into(Path::new(&manifest_directory))
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use super::generate;
    use super::generate_into;
    use crate::umbrella_module_name::UMBRELLA_MODULE_NAME;

    const HOST_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}
}
";

    fn write_crate(directory: &Path, lib_source: &str) {
        let source = directory.join("src");

        fs::create_dir_all(&source).expect("the src directory exists");
        fs::write(source.join("lib.rs"), lib_source).expect("lib.rs is written");
        fs::write(directory.join("Cargo.toml"), "[workspace]\n")
            .expect("the workspace manifest is written");
    }

    fn read_generated(manifest_directory: &Path, name: &str) -> String {
        fs::read_to_string(manifest_directory.join(UMBRELLA_MODULE_NAME).join(name))
            .expect("the generated module exists")
    }

    const FIELDED_SINGLETON_CRATE: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
struct Config {
    name: String,
}
";

    #[test]
    fn generates_the_container_for_the_manifest_crate() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);

        generate_into(host.path()).expect("the crate generates");

        assert!(read_generated(host.path(), "container.rs").contains("struct Container"));
        assert!(read_generated(host.path(), "mod.rs").contains("pub mod container;"));
    }

    #[test]
    fn generates_the_asset_bag_module_from_a_committed_metafile() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);
        fs::write(
            host.path().join("esbuild-meta.json"),
            r#"{"outputs":{"assets/app_ABC.js":{"imports":[],"entryPoint":"src/app.ts"}}}"#,
        )
        .expect("the metafile is written");

        generate_into(host.path()).expect("the crate generates");

        assert!(read_generated(host.path(), "mod.rs").contains("pub mod asset_bag;"));
        assert!(read_generated(host.path(), "asset_bag.rs").contains("macro_rules! asset"));
    }

    const ASSET_RESPONDER_HOST: &str = "\
#[rustfmt::skip]
pub mod margaret;

#[singleton]
struct AssetRoute {
    responder: std::sync::Arc<crate::margaret::asset_bag::asset_responder::AssetResponder>,
}

impl AssetRoute {
    #[constructor]
    fn create(
        responder: std::sync::Arc<crate::margaret::asset_bag::asset_responder::AssetResponder>,
    ) -> anyhow::Result<Self> {}
}
";

    #[test]
    fn serves_every_file_in_the_assets_directory_including_unfingerprinted_ones() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), ASSET_RESPONDER_HOST);
        fs::write(
            host.path().join("esbuild-meta.json"),
            r#"{"outputs":{"assets/app_ABC.js":{"imports":[],"entryPoint":"src/app.ts"}}}"#,
        )
        .expect("the metafile is written");
        let assets = host.path().join("assets");
        fs::create_dir(&assets).expect("the assets directory exists");
        fs::write(assets.join("app_ABC.js"), "console.log(1)")
            .expect("the fingerprinted asset exists");
        fs::write(
            assets.join("service_worker.js"),
            "self.addEventListener('install', () => {})",
        )
        .expect("the service worker exists");

        generate_into(host.path()).expect("the crate generates");

        let responder = read_generated(host.path(), "asset_bag/asset_responder.rs");

        assert!(responder.contains("\"service_worker.js\" =>"));
        assert!(responder.contains("\"no-cache\""));
        assert!(responder.contains("\"app_ABC.js\" =>"));
        assert!(responder.contains("public, max-age=31536000, immutable"));
    }

    #[test]
    fn reports_a_metafile_that_cannot_be_read() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);
        fs::create_dir(host.path().join("esbuild-meta.json"))
            .expect("the blocking directory is created");

        let error = generate_into(host.path()).expect_err("an unreadable metafile is reported");

        assert!(
            error
                .to_string()
                .contains("failed to read the esbuild metafile")
        );
    }

    #[test]
    fn reports_a_manifest_without_a_workspace_root() {
        let host = tempdir().expect("a host crate directory");
        let source = host.path().join("src");

        fs::create_dir_all(&source).expect("the src directory exists");
        fs::write(source.join("lib.rs"), "").expect("lib.rs is written");

        let error = generate_into(host.path())
            .expect_err("a manifest without a workspace root is reported");

        assert!(
            error
                .to_string()
                .contains("no Cargo workspace root was found")
        );
    }

    #[test]
    fn regenerates_cleanly_on_a_second_run() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);

        generate_into(host.path()).expect("the first generation succeeds");
        generate_into(host.path()).expect("the second generation succeeds");

        assert!(read_generated(host.path(), "container.rs").contains("struct Container"));
    }

    #[test]
    fn reads_the_manifest_directory_from_the_environment() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);

        unsafe {
            env::set_var("CARGO_MANIFEST_DIR", host.path());
        }

        generate().expect("the crate generates from the environment");

        assert!(read_generated(host.path(), "container.rs").contains("struct Container"));
    }

    #[test]
    fn reports_a_missing_manifest_directory() {
        unsafe {
            env::remove_var("CARGO_MANIFEST_DIR");
        }

        let error = generate().expect_err("a missing manifest directory is reported");

        assert!(error.to_string().contains("CARGO_MANIFEST_DIR"));
    }

    #[test]
    fn reports_a_generated_directory_that_cannot_be_created() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);
        fs::write(host.path().join(UMBRELLA_MODULE_NAME), "")
            .expect("the blocking file is written");

        let error =
            generate_into(host.path()).expect_err("a blocked generated directory is reported");

        assert!(
            error
                .to_string()
                .contains("failed to create the generated directory")
        );
    }

    #[test]
    fn propagates_a_crate_that_fails_to_build() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), FIELDED_SINGLETON_CRATE);

        let error = generate_into(host.path()).expect_err("an unbuildable crate is reported");

        assert!(
            error
                .to_string()
                .contains("failed to generate the dependency container")
        );
    }

    #[test]
    fn propagates_a_generated_source_that_cannot_be_written() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);
        fs::create_dir_all(host.path().join(UMBRELLA_MODULE_NAME).join("container.rs"))
            .expect("the blocking directory is created");

        let error = generate_into(host.path()).expect_err("a blocked generated source is reported");

        assert!(
            error
                .to_string()
                .contains("failed to write the generated source")
        );
    }
}
