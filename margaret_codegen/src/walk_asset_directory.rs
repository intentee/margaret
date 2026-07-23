use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs;
use std::fs::FileType;
use std::path::Path;
use std::path::PathBuf;

use crate::codegen_error::CodegenError;

fn read_entries(directory: &Path) -> Result<Vec<AssetEntry>, CodegenError> {
    fs::read_dir(directory)
        .and_then(|entries| {
            entries
                .map(|entry| {
                    entry.and_then(|entry| {
                        let path = entry.path();

                        fs::symlink_metadata(&path).map(|metadata| AssetEntry {
                            file_type: metadata.file_type(),
                            path,
                        })
                    })
                })
                .collect::<std::io::Result<Vec<AssetEntry>>>()
        })
        .map_err(|source| CodegenError::ReadAssetDirectory {
            path: directory.to_path_buf(),
            source,
        })
}

fn collect_asset_tails(
    directory: &Path,
    prefix: &str,
    tails: &mut BTreeSet<String>,
) -> Result<(), CodegenError> {
    for AssetEntry { file_type, path } in read_entries(directory)? {
        let Some(name) = path.file_name().and_then(OsStr::to_str) else {
            return Err(CodegenError::NonUtf8AssetPath { path });
        };
        let tail = if prefix.is_empty() {
            name.to_string()
        } else {
            format!("{prefix}/{name}")
        };

        if file_type.is_symlink() {
            return Err(CodegenError::SymlinkAsset { path });
        } else if file_type.is_dir() {
            collect_asset_tails(&path, &tail, tails)?;
        } else if file_type.is_file() {
            tails.insert(tail);
        } else {
            return Err(CodegenError::NonRegularAsset { path });
        }
    }

    Ok(())
}

struct AssetEntry {
    file_type: FileType,
    path: PathBuf,
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
    use std::ffi::OsStr;
    use std::fs;
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::symlink;
    use std::os::unix::net::UnixListener;
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
        let name = OsStr::from_bytes(&[0x66, 0x6f, 0xff]);
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
        let name = OsStr::from_bytes(&[0x66, 0x6f, 0xff]);
        fs::write(nested.join(name), "x").expect("the nested non-utf8 file exists");

        let error =
            walk_asset_directory(directory.path()).expect_err("a nested failure propagates");

        assert!(matches!(
            error,
            CodegenError::NonUtf8AssetPath { path } if path == nested.join(name)
        ));
    }

    #[test]
    fn rejects_a_file_symlink() {
        let directory = tempdir().expect("a temporary assets directory");
        let secret = directory.path().join("secret.txt");
        fs::write(&secret, "top secret").expect("the target file exists");
        let assets = directory.path().join("assets");
        fs::create_dir(&assets).expect("the assets directory exists");
        let link = assets.join("leak.txt");
        symlink(&secret, &link).expect("the file symlink exists");

        let error = walk_asset_directory(&assets).expect_err("a file symlink is rejected");

        assert!(matches!(
            &error,
            CodegenError::SymlinkAsset { path } if *path == link
        ));
        assert!(error.to_string().contains("symbolic link"));
    }

    #[test]
    fn rejects_a_directory_symlink() {
        let directory = tempdir().expect("a temporary assets directory");
        let outside = directory.path().join("outside");
        fs::create_dir(&outside).expect("the target directory exists");
        fs::write(outside.join("secret.js"), "x").expect("the target file exists");
        let assets = directory.path().join("assets");
        fs::create_dir(&assets).expect("the assets directory exists");
        let link = assets.join("escape");
        symlink(&outside, &link).expect("the directory symlink exists");

        let error = walk_asset_directory(&assets).expect_err("a directory symlink is rejected");

        assert!(matches!(
            error,
            CodegenError::SymlinkAsset { path } if path == link
        ));
    }

    #[test]
    fn rejects_a_symlink_cycle() {
        let directory = tempdir().expect("a temporary assets directory");
        let assets = directory.path().join("assets");
        fs::create_dir(&assets).expect("the assets directory exists");
        let link = assets.join("loop");
        symlink(&assets, &link).expect("the cyclic symlink exists");

        let error = walk_asset_directory(&assets).expect_err("a symlink cycle is rejected");

        assert!(matches!(
            error,
            CodegenError::SymlinkAsset { path } if path == link
        ));
    }

    #[test]
    fn rejects_a_non_regular_file() {
        let directory = tempdir().expect("a temporary assets directory");
        let socket_path = directory.path().join("socket");
        let _listener = UnixListener::bind(&socket_path).expect("the unix socket exists");

        let error =
            walk_asset_directory(directory.path()).expect_err("a non-regular file is rejected");

        assert!(matches!(
            &error,
            CodegenError::NonRegularAsset { path } if *path == socket_path
        ));
        assert!(error.to_string().contains("not a regular file"));
    }
}
