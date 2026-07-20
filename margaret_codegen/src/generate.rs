use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

use margaret_attributes::crate_root::CrateRoot;
use margaret_container::provided_singleton::ProvidedSingleton;
use margaret_esbuild_metafile_codegen::esbuild_metafile_provider::esbuild_metafile_provider;

use crate::build::build;
use crate::codegen_error::CodegenError;

fn generate_into(manifest_directory: &Path) -> Result<(), CodegenError> {
    let host_source = manifest_directory.join("src");
    let generated_directory = host_source.join("margaret");
    let crate_root = CrateRoot::new("crate", host_source);

    fs::create_dir_all(&generated_directory).map_err(|source| CodegenError::CreateDirectory {
        path: generated_directory.clone(),
        source,
    })?;

    let umbrella = generated_directory.join("mod.rs");

    if !umbrella.exists() {
        fs::write(&umbrella, "").map_err(|source| CodegenError::WriteSource {
            path: umbrella.clone(),
            source,
        })?;
    }

    let provided_singletons = read_provided_singletons(manifest_directory)?;

    build(&crate_root, provided_singletons)?.write_to(&generated_directory)?;

    println!(
        "cargo:rerun-if-changed={}",
        crate_root.source_directory.display()
    );

    Ok(())
}

fn read_provided_singletons(
    manifest_directory: &Path,
) -> Result<Vec<ProvidedSingleton>, CodegenError> {
    let metafile_path = manifest_directory.join("esbuild-meta.json");

    println!("cargo:rerun-if-changed={}", metafile_path.display());

    match fs::read_to_string(&metafile_path) {
        Ok(contents) => {
            let provider = esbuild_metafile_provider(&contents).map_err(|source| {
                CodegenError::EsbuildMetafileInvalid {
                    path: metafile_path,
                    source,
                }
            })?;

            Ok(vec![provider])
        }
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(Vec::new()),
        Err(source) => Err(CodegenError::EsbuildMetafileRead {
            path: metafile_path,
            source,
        }),
    }
}

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

    use std::os::unix::fs::PermissionsExt;

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
    fn reuses_an_existing_umbrella_stub() {
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
        fs::write(host.path().join("src/margaret"), "").expect("the blocking file is written");

        let error =
            generate_into(host.path()).expect_err("a blocked generated directory is reported");

        assert!(error.to_string().contains("failed to create the generated directory"));
    }

    #[test]
    fn reports_an_umbrella_stub_that_cannot_be_written() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);
        let generated = host.path().join("src/margaret");
        fs::create_dir_all(&generated).expect("the generated directory exists");
        fs::set_permissions(&generated, std::fs::Permissions::from_mode(0o555))
            .expect("the generated directory is made read-only");

        let error = generate_into(host.path()).expect_err("a blocked umbrella stub is reported");

        fs::set_permissions(&generated, std::fs::Permissions::from_mode(0o755))
            .expect("the generated directory is restored to writable");

        assert!(error.to_string().contains("failed to write the generated source"));
    }

    #[test]
    fn propagates_a_crate_that_fails_to_build() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), FIELDED_SINGLETON_CRATE);

        let error = generate_into(host.path()).expect_err("an unbuildable crate is reported");

        assert!(error.to_string().contains("failed to generate the dependency container"));
    }

    #[test]
    fn propagates_a_generated_source_that_cannot_be_written() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);
        fs::create_dir_all(host.path().join("src/margaret/container.rs"))
            .expect("the blocking directory is created");

        let error =
            generate_into(host.path()).expect_err("a blocked generated source is reported");

        assert!(error.to_string().contains("failed to write the generated source"));
    }

    const VALID_METAFILE: &str =
        "{\"outputs\":{\"static/main.js\":{\"imports\":[],\"entryPoint\":\"src/main.ts\",\"inputs\":{}}}}";

    #[test]
    fn bakes_a_present_esbuild_metafile_into_the_container() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);
        fs::write(host.path().join("esbuild-meta.json"), VALID_METAFILE)
            .expect("the esbuild metafile is written");

        generate_into(host.path()).expect("the crate generates with a metafile");

        assert!(
            read_generated(host.path(), "container.rs").contains("esbuild_metafile_esbuild_metafile")
        );
    }

    #[test]
    fn reports_an_invalid_esbuild_metafile() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);
        fs::write(host.path().join("esbuild-meta.json"), "not valid json")
            .expect("the esbuild metafile is written");

        let error = generate_into(host.path()).expect_err("an invalid metafile is reported");

        assert!(error.to_string().contains("is invalid"));
    }

    #[test]
    fn reports_an_unreadable_esbuild_metafile() {
        let host = tempdir().expect("a host crate directory");
        write_crate(host.path(), HOST_CRATE);
        fs::create_dir_all(host.path().join("esbuild-meta.json"))
            .expect("the blocking directory is created");

        let error = generate_into(host.path()).expect_err("an unreadable metafile is reported");

        assert!(error.to_string().contains("failed to read the esbuild metafile"));
    }
}
