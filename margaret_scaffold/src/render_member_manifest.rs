use std::path::PathBuf;

use toml_edit::DocumentMut;
use toml_edit::Item;
use toml_edit::Table;
use toml_edit::value;

use crate::inherited_dependency::InheritedDependency;
use crate::margaret_feature_set::MargaretFeatureSet;
use crate::render_inherited_dependency::render_inherited_dependency;
use crate::render_margaret_dependency::render_margaret_dependency;
use crate::scaffold_error::ScaffoldError;
use crate::scaffolded_file::ScaffoldedFile;
use crate::workspace_dependency_table::WorkspaceDependencyTable;

const MARGARET_PACKAGE_NAME: &str = "margaret";
const SCAFFOLDED_PACKAGE_EDITION: &str = "2024";
const SCAFFOLDED_PACKAGE_VERSION: &str = "0.1.0";

fn package_section(package_name: &str) -> Table {
    let mut package = Table::new();

    package.insert("name", value(package_name));
    package.insert("version", value(SCAFFOLDED_PACKAGE_VERSION));
    package.insert("edition", value(SCAFFOLDED_PACKAGE_EDITION));

    package
}

fn dependencies_section(
    inherited_dependencies: &[InheritedDependency],
    margaret_revision: &str,
    workspace_dependencies: &WorkspaceDependencyTable,
) -> Result<Table, ScaffoldError> {
    let mut dependencies = Table::new();

    for inherited in inherited_dependencies {
        let workspace_entry = workspace_dependencies.require(inherited.name)?;

        dependencies.insert(
            inherited.name,
            Item::Value(render_inherited_dependency(
                inherited.name,
                workspace_entry,
                inherited.additional_features,
            )?),
        );
    }

    dependencies.insert(
        MARGARET_PACKAGE_NAME,
        Item::Value(render_margaret_dependency(
            margaret_revision,
            MargaretFeatureSet::Runtime,
        )),
    );
    dependencies.sort_values();

    Ok(dependencies)
}

fn build_dependencies_section(margaret_revision: &str) -> Table {
    let mut build_dependencies = Table::new();

    build_dependencies.insert(
        MARGARET_PACKAGE_NAME,
        Item::Value(render_margaret_dependency(
            margaret_revision,
            MargaretFeatureSet::Codegen,
        )),
    );

    build_dependencies
}

pub(crate) fn render_member_manifest(
    package_name: &str,
    inherited_dependencies: &[InheritedDependency],
    margaret_revision: &str,
    workspace_dependencies: &WorkspaceDependencyTable,
) -> Result<ScaffoldedFile, ScaffoldError> {
    let mut document = DocumentMut::new();

    document.insert("package", Item::Table(package_section(package_name)));
    document.insert(
        "dependencies",
        Item::Table(dependencies_section(
            inherited_dependencies,
            margaret_revision,
            workspace_dependencies,
        )?),
    );
    document.insert(
        "build-dependencies",
        Item::Table(build_dependencies_section(margaret_revision)),
    );

    Ok(ScaffoldedFile {
        contents: document.to_string(),
        relative_path: PathBuf::from("Cargo.toml"),
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::render_member_manifest;
    use crate::inherited_dependency::InheritedDependency;
    use crate::scaffold_error::ScaffoldError;
    use crate::workspace_dependency_table::WorkspaceDependencyTable;
    use crate::workspace_manifest::WORKSPACE_MANIFEST;

    fn workspace_dependencies() -> WorkspaceDependencyTable {
        WorkspaceDependencyTable::parse(WORKSPACE_MANIFEST).expect("the manifest is parsed")
    }

    #[test]
    fn renders_the_package_the_inherited_dependencies_and_the_codegen_build_dependency() {
        let manifest = render_member_manifest(
            "acme_identity",
            &[InheritedDependency {
                additional_features: &["clock"],
                name: "chrono",
            }],
            "83a27bf",
            &workspace_dependencies(),
        )
        .expect("the manifest is rendered");

        assert_eq!(manifest.relative_path, PathBuf::from("Cargo.toml"));
        assert_eq!(
            manifest.contents,
            concat!(
                "[package]\n",
                "name = \"acme_identity\"\n",
                "version = \"0.1.0\"\n",
                "edition = \"2024\"\n",
                "\n",
                "[dependencies]\n",
                "chrono = { version = \"0.4\", default-features = false, features = [\"std\", \"clock\"] }\n",
                "margaret = { git = \"https://github.com/intentee/margaret\", rev = \"83a27bf\" }\n",
                "\n",
                "[build-dependencies]\n",
                "margaret = { git = \"https://github.com/intentee/margaret\", rev = \"83a27bf\", default-features = false, features = [\"codegen\"] }\n",
            )
        );
    }

    #[test]
    fn reports_an_inherited_dependency_the_workspace_does_not_declare() {
        let error = render_member_manifest(
            "acme_identity",
            &[InheritedDependency {
                additional_features: &[],
                name: "not_a_workspace_dependency",
            }],
            "83a27bf",
            &workspace_dependencies(),
        )
        .expect_err("an undeclared dependency is reported");

        assert!(matches!(
            &error,
            ScaffoldError::WorkspaceDependencyMissing { name }
                if name == "not_a_workspace_dependency"
        ));
    }

    #[test]
    fn reports_an_inherited_dependency_the_workspace_declares_as_a_section() {
        let error = render_member_manifest(
            "acme_identity",
            &[InheritedDependency {
                additional_features: &[],
                name: "anyhow",
            }],
            "83a27bf",
            &WorkspaceDependencyTable::parse(
                "[workspace.dependencies.anyhow]\nversion = \"1.0\"\n",
            )
            .expect("the manifest is parsed"),
        )
        .expect_err("a sectioned dependency is reported");

        assert!(matches!(
            &error,
            ScaffoldError::WorkspaceDependencyMalformed { name } if name == "anyhow"
        ));
    }
}
