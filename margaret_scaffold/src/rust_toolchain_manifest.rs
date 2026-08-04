use std::path::PathBuf;

use crate::scaffolded_file::ScaffoldedFile;

const RUST_TOOLCHAIN_MANIFEST: &str = include_str!("../../rust-toolchain.toml");

#[must_use]
pub(crate) fn rust_toolchain_manifest() -> ScaffoldedFile {
    ScaffoldedFile {
        contents: RUST_TOOLCHAIN_MANIFEST.to_string(),
        relative_path: PathBuf::from("rust-toolchain.toml"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use toml_edit::DocumentMut;
    use toml_edit::Item;

    use super::rust_toolchain_manifest;

    #[test]
    fn pins_the_scaffolded_project_to_the_toolchain_the_framework_is_built_with() {
        let manifest = rust_toolchain_manifest();
        let document = manifest
            .contents
            .parse::<DocumentMut>()
            .expect("the embedded toolchain manifest is valid TOML");
        let channel = document
            .get("toolchain")
            .and_then(Item::as_table)
            .and_then(|toolchain| toolchain.get("channel"))
            .and_then(Item::as_str)
            .expect("the embedded toolchain manifest declares a channel");

        assert_eq!(manifest.relative_path, PathBuf::from("rust-toolchain.toml"));
        assert!(!channel.is_empty());
    }
}
