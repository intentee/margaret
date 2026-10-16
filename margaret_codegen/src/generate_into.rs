use std::fs;
use std::io::ErrorKind;
use std::io::Write;
use std::iter;
use std::path::Path;

use margaret_attributes::crate_root::CrateRoot;
use margaret_sql_identifier::table_namespace::TableNamespace;
use margaret_umbrella_path::constants::UMBRELLA_MODULE_NAME;

use crate::build::build;
use crate::build_output::BuildOutput;
use crate::codegen_error::CodegenError;
use crate::constants::ASSETS_DIRECTORY_NAME;

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

fn write_rerun_directive(directives: &mut impl Write, path: &Path) -> Result<(), CodegenError> {
    writeln!(directives, "cargo:rerun-if-changed={}", path.display()).map_err(|source| {
        CodegenError::WriteRerunDirective {
            path: path.to_path_buf(),
            source,
        }
    })
}

pub(crate) fn generate_into(
    manifest_directory: &Path,
    namespace: TableNamespace,
    directives: &mut impl Write,
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
    let BuildOutput {
        asset_bag_inputs,
        code,
    } = build(
        &crate_root,
        metafile_contents.as_deref(),
        &assets_directory,
        namespace,
    )?;

    code.write_to(&generated_directory)?;

    iter::once(crate_root.source_directory.as_path())
        .chain(asset_bag_inputs.read_paths(&metafile_path, &assets_directory))
        .try_for_each(|path| write_rerun_directive(directives, path))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::OpenOptions;
    use std::path::Path;
    use std::path::PathBuf;

    use margaret_attributes_tests::source_crate::SourceCrate;
    use margaret_container::container_error::ContainerError;
    use margaret_sql_identifier::table_namespace::TableNamespace;
    use margaret_umbrella_path::constants::UMBRELLA_MODULE_NAME;

    use super::generate_into;
    use crate::codegen_error::CodegenError;

    const HOST_CRATE: &str = "use margaret::framework::macros::console_command;\nuse margaret::framework::macros::constructor;\nuse margaret::framework::macros::process;\nuse margaret::framework::macros::singleton;\n\
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

    const ASSET_MACRO_HOST: &str = "use margaret::framework::macros::console_command;\nuse margaret::framework::macros::constructor;\nuse margaret::framework::macros::process;\nuse margaret::framework::macros::singleton;\n\
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

    const ASSET_RESPONDER_HOST: &str = "use margaret::framework::macros::console_command;\nuse margaret::framework::macros::constructor;\nuse margaret::framework::macros::process;\nuse margaret::framework::macros::singleton;\n\
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

    const FIELDED_SINGLETON_CRATE: &str = "use margaret::framework::macros::singleton;\n\
#[singleton]
struct Config {
    name: String,
}
";

    const METAFILE: &str =
        r#"{"outputs":{"assets/app_ABC.js":{"imports":[],"entryPoint":"src/app.ts"}}}"#;

    fn generated(host: &SourceCrate) -> Result<(), CodegenError> {
        generate_into(host.root(), TableNamespace::Application, &mut Vec::new())
    }

    fn rerun_directives(host: &SourceCrate) -> String {
        let mut directives = Vec::new();

        generate_into(host.root(), TableNamespace::Application, &mut directives)
            .expect("the crate generates");

        String::from_utf8(directives).expect("the directives are text")
    }

    fn rerun_directive(path: &Path) -> String {
        format!("cargo:rerun-if-changed={}\n", path.display())
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

        assert!(read_generated(&host, "container.rs").contains("pub mod build;"));
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
    fn watches_only_the_sources_of_a_crate_without_an_asset_bag() {
        let host = SourceCrate::new(HOST_CRATE);

        assert_eq!(
            rerun_directives(&host),
            rerun_directive(&host.root().join("src"))
        );
    }

    #[test]
    fn watches_the_metafile_of_a_crate_importing_the_asset_macro() {
        let host = SourceCrate::new(ASSET_MACRO_HOST);

        write_metafile(host.root());

        assert_eq!(
            rerun_directives(&host),
            [
                rerun_directive(&host.root().join("src")),
                rerun_directive(&host.root().join("esbuild-meta.json")),
            ]
            .concat()
        );
    }

    #[test]
    fn watches_the_metafile_and_the_assets_of_a_crate_serving_them() {
        let host = SourceCrate::new(ASSET_RESPONDER_HOST);

        let assets = host.root().join("assets");

        write_metafile(host.root());
        fs::create_dir(&assets).expect("the assets directory exists");
        fs::write(assets.join("app_ABC.js"), "console.log(1)")
            .expect("the fingerprinted asset exists");

        assert_eq!(
            rerun_directives(&host),
            [
                rerun_directive(&host.root().join("src")),
                rerun_directive(&host.root().join("esbuild-meta.json")),
                rerun_directive(&assets),
            ]
            .concat()
        );
    }

    #[test]
    fn reports_a_rerun_directive_that_cannot_be_written() {
        let host = SourceCrate::new(HOST_CRATE);
        let mut directives = OpenOptions::new()
            .write(true)
            .open("/dev/full")
            .expect("/dev/full opens for writing");

        assert!(matches!(
            generate_into(host.root(), TableNamespace::Application, &mut directives),
            Err(CodegenError::WriteRerunDirective { path, .. }) if path == host.root().join("src")
        ));
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
    fn regenerates_over_its_previous_output_so_every_rebuild_of_the_crate_succeeds() {
        let host = SourceCrate::new(HOST_CRATE);

        generated(&host).expect("the first generation succeeds");
        generated(&host).expect("the second generation succeeds");

        assert!(read_generated(&host, "container.rs").contains("pub mod build;"));
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
