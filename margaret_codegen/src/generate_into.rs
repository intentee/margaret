use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use margaret_attributes::crate_root::CrateRoot;
use margaret_sql_identifier::table_namespace::TableNamespace;
use margaret_umbrella_path::umbrella_module_name::UMBRELLA_MODULE_NAME;

use crate::assets_directory_name::ASSETS_DIRECTORY_NAME;
use crate::build::build;
use crate::codegen_error::CodegenError;

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

pub(crate) fn generate_into(
    manifest_directory: &Path,
    namespace: TableNamespace,
) -> Result<(), CodegenError> {
    let host_source = manifest_directory.join("src");
    let generated_directory = manifest_directory.join(UMBRELLA_MODULE_NAME);
    let metafile_path = manifest_directory.join("esbuild-meta.json");
    let assets_directory = manifest_directory.join(ASSETS_DIRECTORY_NAME);
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
        namespace,
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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    use margaret_attributes_tests::source_crate::SourceCrate;
    use margaret_container::container_error::ContainerError;
    use margaret_sql_identifier::table_namespace::TableNamespace;
    use margaret_umbrella_path::umbrella_module_name::UMBRELLA_MODULE_NAME;

    use super::generate_into;
    use crate::codegen_error::CodegenError;

    const HOST_CRATE: &str = "\
#[singleton]
#[console_command(name = \"inspect\")]
struct Config;

impl Config {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<CommandOutcome> {}
}
";

    const ASSET_MACRO_HOST: &str = "\
use crate::margaret::asset_bag::asset;

#[singleton]
#[console_command(name = \"inspect\")]
struct Config;

impl Config {
    #[constructor]
    fn create() -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<CommandOutcome> {}
}
";

    const ASSET_RESPONDER_HOST: &str = "\
#[singleton]
#[console_command(name = \"assets\")]
struct AssetRoute {
    responder: std::sync::Arc<crate::margaret::asset_bag::asset_responder::AssetResponder>,
}

impl AssetRoute {
    #[constructor]
    fn create(
        responder: std::sync::Arc<crate::margaret::asset_bag::asset_responder::AssetResponder>,
    ) -> anyhow::Result<Self> {}

    #[process]
    fn run(&self) -> anyhow::Result<CommandOutcome> {}
}
";

    const FIELDED_SINGLETON_CRATE: &str = "\
#[singleton]
struct Config {
    name: String,
}
";

    const METAFILE: &str =
        r#"{"outputs":{"assets/app_ABC.js":{"imports":[],"entryPoint":"src/app.ts"}}}"#;

    fn generated(host: &SourceCrate) -> Result<(), CodegenError> {
        generate_into(host.root(), TableNamespace::Application)
    }

    fn generated_path(host: &SourceCrate, name: &str) -> PathBuf {
        host.root().join(UMBRELLA_MODULE_NAME).join(name)
    }

    fn read_generated(host: &SourceCrate, name: &str) -> String {
        fs::read_to_string(generated_path(host, name)).expect("the generated module exists")
    }

    fn write_metafile(root: &Path) {
        fs::write(root.join("esbuild-meta.json"), METAFILE).expect("the metafile is written");
    }

    #[test]
    fn generates_the_container_into_the_umbrella_directory_of_the_manifest() {
        let host = SourceCrate::new(HOST_CRATE);

        generated(&host).expect("the crate generates");

        assert!(read_generated(&host, "container.rs").contains("struct Container"));
        assert!(read_generated(&host, "mod.rs").contains("pub mod container;"));
    }

    #[test]
    fn generates_the_asset_bag_module_from_a_committed_metafile() {
        let host = SourceCrate::new(ASSET_MACRO_HOST);

        write_metafile(host.root());
        generated(&host).expect("the crate generates");

        assert!(read_generated(&host, "mod.rs").contains("pub mod asset_bag;"));
        assert!(read_generated(&host, "asset_bag.rs").contains("macro_rules! asset"));
    }

    #[test]
    fn serves_every_file_in_the_assets_directory_including_unfingerprinted_ones() {
        let host = SourceCrate::new(ASSET_RESPONDER_HOST);
        let assets = host.root().join("assets");

        write_metafile(host.root());
        fs::create_dir(&assets).expect("the assets directory exists");
        fs::write(assets.join("app_ABC.js"), "console.log(1)")
            .expect("the fingerprinted asset exists");
        fs::write(
            assets.join("service_worker.js"),
            "self.addEventListener('install', () => {})",
        )
        .expect("the service worker exists");
        generated(&host).expect("the crate generates");

        let responder = read_generated(&host, "asset_bag/asset_responder.rs");

        assert!(responder.contains("\"service_worker.js\" =>"));
        assert!(responder.contains("\"no-cache\""));
        assert!(responder.contains("\"app_ABC.js\" =>"));
        assert!(responder.contains("public, max-age=31536000, immutable"));
    }

    #[test]
    fn reports_a_metafile_that_cannot_be_read() {
        let host = SourceCrate::new(HOST_CRATE);
        let metafile = host.root().join("esbuild-meta.json");

        fs::create_dir(&metafile).expect("the blocking directory is created");

        assert!(matches!(
            generated(&host),
            Err(CodegenError::ReadMetafile { path, .. }) if path == metafile
        ));
    }

    #[test]
    fn regenerates_cleanly_on_a_second_run() {
        let host = SourceCrate::new(HOST_CRATE);

        generated(&host).expect("the first generation succeeds");
        generated(&host).expect("the second generation succeeds");

        assert!(read_generated(&host, "container.rs").contains("struct Container"));
    }

    #[test]
    fn reports_a_generated_directory_that_cannot_be_created() {
        let host = SourceCrate::new(HOST_CRATE);
        let blocked = host.root().join(UMBRELLA_MODULE_NAME);

        fs::write(&blocked, "").expect("the blocking file is written");

        assert!(matches!(
            generated(&host),
            Err(CodegenError::CreateDirectory { path, .. }) if path == blocked
        ));
    }

    #[test]
    fn propagates_a_crate_that_fails_to_build() {
        let host = SourceCrate::new(FIELDED_SINGLETON_CRATE);

        assert!(matches!(
            generated(&host),
            Err(CodegenError::Container {
                source: ContainerError::SingletonRequiresConstructor { singleton, field_count: 1 }
            }) if singleton == "crate::Config"
        ));
    }

    #[test]
    fn propagates_a_generated_source_that_cannot_be_written() {
        let host = SourceCrate::new(HOST_CRATE);
        let blocked = generated_path(&host, "container.rs");

        fs::create_dir_all(&blocked).expect("the blocking directory is created");

        assert!(matches!(
            generated(&host),
            Err(CodegenError::WriteSource { path, .. }) if path == blocked
        ));
    }
}
