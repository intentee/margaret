use crate::build_artifacts_gitignore::build_artifacts_gitignore;
use crate::project_name::ProjectName;
use crate::render_base_service::render_base_service;
use crate::render_identity_service::render_identity_service;
use crate::render_workspace_manifest::render_workspace_manifest;
use crate::rust_toolchain_manifest::rust_toolchain_manifest;
use crate::scaffold_error::ScaffoldError;
use crate::scaffolded_file::ScaffoldedFile;
use crate::workspace_dependency_table::WorkspaceDependencyTable;

pub(crate) fn render_project(
    margaret_revision: &str,
    project_name: &ProjectName,
    workspace_manifest: &str,
) -> Result<Vec<ScaffoldedFile>, ScaffoldError> {
    let workspace_dependencies = WorkspaceDependencyTable::parse(workspace_manifest)?;
    let mut files = vec![
        render_workspace_manifest(project_name),
        build_artifacts_gitignore(),
        rust_toolchain_manifest(),
    ];

    files.extend(render_identity_service(
        margaret_revision,
        project_name,
        &workspace_dependencies,
    )?);
    files.extend(render_base_service(
        margaret_revision,
        project_name,
        &workspace_dependencies,
    )?);

    Ok(files)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::render_project;
    use crate::project_name::ProjectName;
    use crate::scaffold_error::ScaffoldError;
    use crate::workspace_manifest::WORKSPACE_MANIFEST;

    const IDENTITY_ONLY_DEPENDENCIES: &str = concat!(
        "[workspace.dependencies]\n",
        "anyhow = \"1.0\"\n",
        "async-trait = \"0.1\"\n",
        "chrono = \"0.4\"\n",
        "clap = \"4.5\"\n",
        "spiffe = \"0.6\"\n",
        "tokio = \"1.52\"\n",
        "tokio-util = \"0.7\"\n",
        "trzcina = \"0.4\"\n",
    );

    fn project_name() -> ProjectName {
        ProjectName::new("acme".to_string())
    }

    #[test]
    fn lays_out_the_workspace_root_next_to_both_member_crates() {
        let paths: Vec<PathBuf> = render_project("83a27bf", &project_name(), WORKSPACE_MANIFEST)
            .expect("the project is rendered")
            .into_iter()
            .map(|file| file.relative_path)
            .collect();

        assert_eq!(
            &paths[..3],
            &[
                PathBuf::from("Cargo.toml"),
                PathBuf::from(".gitignore"),
                PathBuf::from("rust-toolchain.toml"),
            ]
        );
        assert!(paths.contains(&PathBuf::from("acme_identity").join("Cargo.toml")));
        assert!(paths.contains(&PathBuf::from("acme_base").join("Cargo.toml")));
    }

    #[test]
    fn never_renders_the_same_relative_path_twice() {
        let mut paths: Vec<PathBuf> =
            render_project("83a27bf", &project_name(), WORKSPACE_MANIFEST)
                .expect("the project is rendered")
                .into_iter()
                .map(|file| file.relative_path)
                .collect();
        let rendered = paths.len();

        paths.sort();
        paths.dedup();

        assert_eq!(paths.len(), rendered);
    }

    #[test]
    fn reports_a_workspace_manifest_that_cannot_be_read() {
        let error = render_project("83a27bf", &project_name(), "not = = toml")
            .expect_err("an invalid workspace manifest is reported");

        assert!(
            error
                .to_string()
                .contains("the workspace manifest is not valid TOML")
        );
    }

    #[test]
    fn reports_a_dependency_the_identity_service_cannot_inherit() {
        let error = render_project("83a27bf", &project_name(), "[workspace.dependencies]\n")
            .expect_err("a missing identity dependency is reported");

        assert!(matches!(
            &error,
            ScaffoldError::WorkspaceDependencyMissing { name } if name == "anyhow"
        ));
    }

    #[test]
    fn reports_a_dependency_only_the_base_service_needs() {
        let error = render_project("83a27bf", &project_name(), IDENTITY_ONLY_DEPENDENCIES)
            .expect_err("a missing base dependency is reported");

        assert!(matches!(
            &error,
            ScaffoldError::WorkspaceDependencyMissing { name } if name == "reqwest"
        ));
    }
}
