use std::sync::Arc;

use esbuild_metafile::esbuild_metafile::EsbuildMetafile;
use margaret_macros::constructor;
use margaret_macros::singleton;

#[singleton]
pub struct AssetManifest {
    esbuild_metafile: Arc<EsbuildMetafile>,
}

impl AssetManifest {
    #[constructor]
    #[must_use]
    pub fn create(esbuild_metafile: Arc<EsbuildMetafile>) -> Self {
        Self { esbuild_metafile }
    }

    #[must_use]
    pub fn outputs_for(&self, input_path: &str) -> Option<Vec<String>> {
        self.esbuild_metafile.find_outputs_for_input(input_path)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::sync::Arc;

    use esbuild_metafile::esbuild_metafile::EsbuildMetafile;

    use super::AssetManifest;

    #[test]
    fn resolves_outputs_through_the_injected_metafile() {
        let metafile = EsbuildMetafile::from_str(
            r#"{"outputs":{"static/main.js":{"imports":[],"entryPoint":"resources/ts/main.ts","inputs":{}}}}"#,
        )
        .expect("the metafile parses");
        let manifest = AssetManifest::create(Arc::new(metafile));

        assert_eq!(
            manifest.outputs_for("resources/ts/main.ts"),
            Some(vec!["static/main.js".to_string()])
        );
        assert!(manifest.outputs_for("resources/ts/missing.ts").is_none());
    }
}
