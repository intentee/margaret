use toml_edit::Array;
use toml_edit::InlineTable;
use toml_edit::Value;

use crate::margaret_feature_set::MargaretFeatureSet;
use crate::margaret_git_url::MARGARET_GIT_URL;

const CODEGEN_FEATURE: &str = "codegen";

#[must_use]
pub(crate) fn render_margaret_dependency(revision: &str, feature_set: MargaretFeatureSet) -> Value {
    let mut inline = InlineTable::new();

    inline.insert("git", Value::from(MARGARET_GIT_URL));
    inline.insert("rev", Value::from(revision));

    match feature_set {
        MargaretFeatureSet::Codegen => {
            let mut features = Array::new();

            features.push(CODEGEN_FEATURE);
            inline.insert("default-features", Value::from(false));
            inline.insert("features", Value::Array(features));
        }
        MargaretFeatureSet::Runtime => {}
    }

    Value::InlineTable(inline).decorated(" ", "")
}

#[cfg(test)]
mod tests {
    use super::render_margaret_dependency;
    use crate::margaret_feature_set::MargaretFeatureSet;

    #[test]
    fn pins_the_runtime_dependency_to_the_requested_revision() {
        assert_eq!(
            render_margaret_dependency("83a27bf", MargaretFeatureSet::Runtime).to_string(),
            " { git = \"https://github.com/intentee/margaret\", rev = \"83a27bf\" }"
        );
    }

    #[test]
    fn narrows_the_build_dependency_to_the_codegen_feature() {
        assert_eq!(
            render_margaret_dependency("83a27bf", MargaretFeatureSet::Codegen).to_string(),
            " { git = \"https://github.com/intentee/margaret\", rev = \"83a27bf\", default-features = false, features = [\"codegen\"] }"
        );
    }
}
