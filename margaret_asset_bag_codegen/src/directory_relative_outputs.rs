use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::asset_bag_codegen_error::AssetBagCodegenError;

pub(crate) fn directory_relative_outputs(
    output_paths: &BTreeSet<String>,
) -> Result<BTreeMap<String, String>, AssetBagCodegenError> {
    let mut directory: Option<&str> = None;
    let mut relative_outputs: BTreeMap<String, String> = BTreeMap::new();

    for output_path in output_paths {
        let Some((candidate, tail)) = output_path.split_once('/') else {
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

        relative_outputs.insert(output_path.clone(), tail.to_string());
    }

    match directory {
        Some(_) => Ok(relative_outputs),
        None => Err(AssetBagCodegenError::EmptyMetafile),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::directory_relative_outputs;
    use crate::asset_bag_codegen_error::AssetBagCodegenError;

    fn paths(paths: &[&str]) -> BTreeSet<String> {
        paths.iter().map(|path| (*path).to_string()).collect()
    }

    #[test]
    fn maps_each_output_to_its_directory_relative_tail() {
        let relative_outputs = directory_relative_outputs(&paths(&[
            "assets/app_ABC.js",
            "assets/nested/chunk_ABC.js",
        ]))
        .expect("the shared directory resolves");

        assert_eq!(
            relative_outputs
                .get("assets/app_ABC.js")
                .map(String::as_str),
            Some("app_ABC.js")
        );
        assert_eq!(
            relative_outputs
                .get("assets/nested/chunk_ABC.js")
                .map(String::as_str),
            Some("nested/chunk_ABC.js")
        );
    }

    #[test]
    fn rejects_an_output_without_a_directory() {
        assert!(matches!(
            directory_relative_outputs(&paths(&["app_ABC.js"])),
            Err(AssetBagCodegenError::OutputMissingDirectory { output }) if output == "app_ABC.js"
        ));
    }

    #[test]
    fn rejects_outputs_spanning_multiple_directories() {
        assert!(matches!(
            directory_relative_outputs(&paths(&["assets/app_ABC.js", "static/app_ABC.css"])),
            Err(AssetBagCodegenError::InconsistentOutputDirectory { first, second })
                if first == "assets" && second == "static"
        ));
    }

    #[test]
    fn rejects_a_metafile_without_outputs() {
        assert_eq!(
            directory_relative_outputs(&BTreeSet::new())
                .expect_err("an empty metafile is rejected")
                .to_string(),
            "the esbuild metafile declares no outputs"
        );
    }
}
