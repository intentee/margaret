use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::codegen_error::CodegenError;

fn collect_asset_tails(
    directory: &Path,
    prefix: &str,
    tails: &mut BTreeSet<String>,
) -> Result<(), CodegenError> {
    let entries: Vec<fs::DirEntry> = fs::read_dir(directory)
        .and_then(Iterator::collect)
        .map_err(|source| CodegenError::ReadAssetDirectory {
            path: directory.to_path_buf(),
            source,
        })?;

    for entry in entries {
        let path = entry.path();
        let file_name = entry.file_name();
        let Some(name) = file_name.to_str() else {
            return Err(CodegenError::NonUtf8AssetPath { path });
        };
        let tail = if prefix.is_empty() {
            name.to_string()
        } else {
            format!("{prefix}/{name}")
        };

        if path.is_dir() {
            collect_asset_tails(&path, &tail, tails)?;
        } else {
            tails.insert(tail);
        }
    }

    Ok(())
}

pub(crate) fn walk_asset_directory(
    assets_directory: &Path,
) -> Result<BTreeSet<String>, CodegenError> {
    let mut tails = BTreeSet::new();

    collect_asset_tails(assets_directory, "", &mut tails)?;

    Ok(tails)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::ffi::OsStrExt;
    use std::path::Path;

    use tempfile::tempdir;

    use super::walk_asset_directory;
    use crate::codegen_error::CodegenError;

    #[test]
    fn collects_flat_and_nested_files_as_directory_relative_tails() {
        let directory = tempdir().expect("a temporary assets directory");
        let nested = directory.path().join("chunks");
        fs::create_dir(&nested).expect("the nested directory exists");
        fs::write(directory.path().join("app_A1B2C3D4.js"), "a").expect("the flat file exists");
        fs::write(directory.path().join("service_worker.js"), "b")
            .expect("the un-fingerprinted file exists");
        fs::write(nested.join("chunk_E5F6G7H8.js"), "c").expect("the nested file exists");

        let tails = walk_asset_directory(directory.path()).expect("the directory is walked");

        assert_eq!(
            tails,
            [
                "app_A1B2C3D4.js".to_string(),
                "chunks/chunk_E5F6G7H8.js".to_string(),
                "service_worker.js".to_string(),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn returns_an_empty_set_for_an_empty_directory() {
        let directory = tempdir().expect("a temporary assets directory");

        let tails = walk_asset_directory(directory.path()).expect("the directory is walked");

        assert!(tails.is_empty());
    }

    #[test]
    fn reports_a_directory_that_cannot_be_read() {
        let error = walk_asset_directory(Path::new("/does/not/exist"))
            .expect_err("a missing directory is reported");

        assert!(matches!(
            &error,
            CodegenError::ReadAssetDirectory { path, .. } if path == Path::new("/does/not/exist")
        ));
        assert!(
            error
                .to_string()
                .contains("failed to read the asset directory")
        );
    }

    #[test]
    fn rejects_a_non_utf8_asset_file_name() {
        let directory = tempdir().expect("a temporary assets directory");
        let name = std::ffi::OsStr::from_bytes(&[0x66, 0x6f, 0xff]);
        fs::write(directory.path().join(name), "x").expect("the non-utf8 file exists");

        let error =
            walk_asset_directory(directory.path()).expect_err("a non-utf8 file name is rejected");

        assert!(matches!(
            &error,
            CodegenError::NonUtf8AssetPath { path } if *path == directory.path().join(name)
        ));
        assert!(error.to_string().contains("is not valid UTF-8"));
    }

    #[test]
    fn propagates_a_failure_from_a_nested_directory() {
        let directory = tempdir().expect("a temporary assets directory");
        let nested = directory.path().join("chunks");
        fs::create_dir(&nested).expect("the nested directory exists");
        let name = std::ffi::OsStr::from_bytes(&[0x66, 0x6f, 0xff]);
        fs::write(nested.join(name), "x").expect("the nested non-utf8 file exists");

        let error =
            walk_asset_directory(directory.path()).expect_err("a nested failure propagates");

        assert!(matches!(
            error,
            CodegenError::NonUtf8AssetPath { path } if path == nested.join(name)
        ));
    }
}
