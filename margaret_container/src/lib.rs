mod collection_table;
pub mod container_error;
mod container_plan;
mod dependency_kind;
mod find_constructor;
pub mod generated_container;
mod path_text;
mod provided_type;
mod provider;
mod raw_target;
mod render;
mod resolution;
mod topological_order;
mod type_text;

use std::path::Path;

use margaret_attributes::attribute_index::AttributeIndex;

use crate::container_error::ContainerError;
use crate::container_plan::build_plan;
use crate::generated_container::GeneratedContainer;
use crate::render::render;
use crate::topological_order::topological_order;

pub fn generate_container_source(
    crate_name: &str,
    source_directory: &Path,
) -> Result<GeneratedContainer, ContainerError> {
    let index = AttributeIndex::from_crate_root(crate_name, source_directory)?;
    let plan = build_plan(&index)?;
    let order = topological_order(&plan.providers, &plan.collections)?;

    Ok(GeneratedContainer::new(render(&plan, &order)))
}

pub fn build(manifest_directory: impl AsRef<Path>) -> Result<(), ContainerError> {
    let source_directory = manifest_directory.as_ref().join("src");
    let container_path = source_directory.join("container.rs");

    if !container_path.exists() {
        std::fs::write(&container_path, "").expect("the container stub is written");
    }

    generate_container_source("crate", &source_directory)?
        .write_if_changed(&container_path)
        .expect("the generated container is written");

    println!("cargo:rerun-if-changed={}", source_directory.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use crate::build;

    const SINGLETON_CRATE: &str = "\
#[singleton]
struct Config;

impl Config {
    #[constructor]
    fn new() -> Self {}
}

pub mod container;
";

    fn crate_with(lib_source: &str) -> TempDir {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        directory
    }

    #[test]
    fn writes_the_container_when_absent() {
        let crate_directory = crate_with(SINGLETON_CRATE);

        build(crate_directory.path()).expect("the container is built");

        let generated = fs::read_to_string(crate_directory.path().join("src/container.rs"))
            .expect("the generated container exists");

        assert!(generated.contains("struct Container"));
    }

    #[test]
    fn rewrites_an_existing_container() {
        let crate_directory = crate_with(SINGLETON_CRATE);
        let container_path = crate_directory.path().join("src/container.rs");
        fs::write(&container_path, "").expect("an empty container is seeded");

        build(crate_directory.path()).expect("the container is rebuilt");

        let generated =
            fs::read_to_string(&container_path).expect("the generated container exists");

        assert!(generated.contains("struct Container"));
    }

    #[test]
    fn propagates_generation_failure() {
        let crate_directory = crate_with("use other::*;\n\npub mod container;\n");

        let message = build(crate_directory.path())
            .expect_err("a glob import fails the build")
            .to_string();

        assert!(message.contains("failed to index"));
    }
}
