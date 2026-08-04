use std::path::PathBuf;

use toml_edit::Array;
use toml_edit::DocumentMut;
use toml_edit::Item;
use toml_edit::Table;
use toml_edit::value;

use crate::project_name::ProjectName;
use crate::scaffolded_file::ScaffoldedFile;

const WORKSPACE_RESOLVER: &str = "3";

#[must_use]
pub(crate) fn render_workspace_manifest(project_name: &ProjectName) -> ScaffoldedFile {
    let mut members = Array::new();

    members.push(project_name.base_crate());
    members.push(project_name.identity_crate());

    let mut workspace = Table::new();

    workspace.insert("resolver", value(WORKSPACE_RESOLVER));
    workspace.insert("members", value(members));

    let mut document = DocumentMut::new();

    document.insert("workspace", Item::Table(workspace));

    ScaffoldedFile {
        contents: document.to_string(),
        relative_path: PathBuf::from("Cargo.toml"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::render_workspace_manifest;
    use crate::project_name::ProjectName;

    #[test]
    fn declares_both_members_of_the_scaffolded_workspace() {
        let manifest = render_workspace_manifest(&ProjectName::new("acme".to_string()));

        assert_eq!(manifest.relative_path, PathBuf::from("Cargo.toml"));
        assert_eq!(
            manifest.contents,
            concat!(
                "[workspace]\n",
                "resolver = \"3\"\n",
                "members = [\"acme_base\", \"acme_identity\"]\n",
            )
        );
    }
}
