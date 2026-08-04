use toml_edit::DocumentMut;
use toml_edit::Item;
use toml_edit::Table;

use crate::scaffold_error::ScaffoldError;

#[derive(Debug)]
pub(crate) struct WorkspaceDependencyTable {
    dependencies: Table,
}

impl WorkspaceDependencyTable {
    pub(crate) fn parse(manifest: &str) -> Result<Self, ScaffoldError> {
        let document = manifest
            .parse::<DocumentMut>()
            .map_err(|source| ScaffoldError::WorkspaceManifestMalformed { source })?;
        let dependencies = document
            .get("workspace")
            .and_then(Item::as_table)
            .and_then(|workspace| workspace.get("dependencies"))
            .and_then(Item::as_table)
            .ok_or(ScaffoldError::WorkspaceDependencyTableMissing)?;

        Ok(Self {
            dependencies: dependencies.clone(),
        })
    }

    pub(crate) fn require(&self, name: &str) -> Result<&Item, ScaffoldError> {
        self.dependencies
            .get(name)
            .ok_or_else(|| ScaffoldError::WorkspaceDependencyMissing {
                name: name.to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::WorkspaceDependencyTable;
    use crate::scaffold_error::ScaffoldError;
    use crate::workspace_manifest::WORKSPACE_MANIFEST;

    #[test]
    fn exposes_a_dependency_declared_by_the_embedded_workspace_manifest() {
        let table =
            WorkspaceDependencyTable::parse(WORKSPACE_MANIFEST).expect("the manifest is parsed");

        assert!(
            table
                .require("anyhow")
                .expect("anyhow is declared by the workspace")
                .as_value()
                .is_some()
        );
    }

    #[test]
    fn reports_a_dependency_the_workspace_does_not_declare() {
        let table =
            WorkspaceDependencyTable::parse(WORKSPACE_MANIFEST).expect("the manifest is parsed");

        let error = table
            .require("not_a_workspace_dependency")
            .expect_err("an undeclared dependency is reported");

        assert!(matches!(
            &error,
            ScaffoldError::WorkspaceDependencyMissing { name }
                if name == "not_a_workspace_dependency"
        ));
    }

    #[test]
    fn reports_a_manifest_that_is_not_valid_toml() {
        let error = WorkspaceDependencyTable::parse("not = = toml")
            .expect_err("an invalid manifest is reported");

        assert!(
            error
                .to_string()
                .contains("the workspace manifest is not valid TOML")
        );
    }

    #[test]
    fn reports_a_manifest_without_a_workspace_dependency_table() {
        let error = WorkspaceDependencyTable::parse("[workspace]\nmembers = []\n")
            .expect_err("a manifest without workspace dependencies is reported");

        assert_eq!(
            error.to_string(),
            "the workspace manifest declares no [workspace.dependencies] table"
        );
    }
}
