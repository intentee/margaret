use std::collections::BTreeSet;

use crate::asset_bag_codegen_error::AssetBagCodegenError;

pub(crate) fn output_directory(
    output_paths: &BTreeSet<String>,
) -> Result<String, AssetBagCodegenError> {
    let mut directory: Option<&str> = None;

    for output_path in output_paths {
        let Some((candidate, _rest)) = output_path.split_once('/') else {
            return Err(AssetBagCodegenError::OutputMissingDirectory {
                output: output_path.clone(),
            });
        };

        match directory {
            None => directory = Some(candidate),
            Some(existing) if existing == candidate => {}
            Some(existing) => {
                return Err(AssetBagCodegenError::InconsistentOutputDirectory {
                    first: existing.to_string(),
                    second: candidate.to_string(),
                });
            }
        }
    }

    match directory {
        Some(directory) => Ok(directory.to_string()),
        None => Err(AssetBagCodegenError::EmptyMetafile),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::output_directory;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;

    fn paths(paths: &[&str]) -> BTreeSet<String> {
        paths.iter().map(|path| (*path).to_string()).collect()
    }

    #[test]
    fn derives_the_shared_output_directory() {
        assert_eq!(
            output_directory(&paths(&["assets/app_ABC.js", "assets/nested/chunk_ABC.js"]))
                .expect("a shared directory is derived"),
            "assets"
        );
    }

    #[test]
    fn rejects_an_output_without_a_directory() {
        assert!(matches!(
            output_directory(&paths(&["app_ABC.js"])),
            Err(AssetBagCodegenError::OutputMissingDirectory { output }) if output == "app_ABC.js"
        ));
    }

    #[test]
    fn rejects_outputs_spanning_multiple_directories() {
        assert!(matches!(
            output_directory(&paths(&["assets/app_ABC.js", "static/app_ABC.css"])),
            Err(AssetBagCodegenError::InconsistentOutputDirectory { first, second })
                if first == "assets" && second == "static"
        ));
    }

    #[test]
    fn rejects_a_metafile_without_outputs() {
        assert_eq!(
            output_directory(&BTreeSet::new())
                .expect_err("an empty metafile is rejected")
                .to_string(),
            "the esbuild metafile declares no outputs"
        );
    }
}
