use toml_edit::Array;
use toml_edit::InlineTable;
use toml_edit::Item;
use toml_edit::Value;

use crate::scaffold_error::ScaffoldError;

fn inline_table_for(name: &str, workspace_entry: &Item) -> Result<InlineTable, ScaffoldError> {
    match workspace_entry.as_value() {
        Some(Value::String(version)) => {
            let mut inline = InlineTable::new();

            inline.insert("version", Value::String(version.clone()));

            Ok(inline)
        }
        Some(Value::InlineTable(inline)) => Ok(inline.clone()),
        _ => Err(ScaffoldError::WorkspaceDependencyMalformed {
            name: name.to_string(),
        }),
    }
}

fn extended_features(inline: &InlineTable, additional_features: &[&str]) -> Array {
    let mut features = match inline.get("features").and_then(Value::as_array) {
        Some(declared) => declared.clone(),
        None => Array::new(),
    };

    for feature in additional_features {
        features.push(*feature);
    }

    features
}

pub(crate) fn render_inherited_dependency(
    name: &str,
    workspace_entry: &Item,
    additional_features: &[&str],
) -> Result<Value, ScaffoldError> {
    if additional_features.is_empty() {
        return workspace_entry
            .as_value()
            .map(|value| value.clone().decorated(" ", ""))
            .ok_or_else(|| ScaffoldError::WorkspaceDependencyMalformed {
                name: name.to_string(),
            });
    }

    let mut inline = inline_table_for(name, workspace_entry)?;
    let features = extended_features(&inline, additional_features);

    inline.insert("features", Value::Array(features));
    inline.fmt();

    Ok(Value::InlineTable(inline).decorated(" ", ""))
}

#[cfg(test)]
mod tests {
    use toml_edit::DocumentMut;
    use toml_edit::Item;

    use super::render_inherited_dependency;
    use crate::scaffold_error::ScaffoldError;

    fn workspace_entry(declaration: &str) -> Item {
        declaration
            .parse::<DocumentMut>()
            .expect("the declaration is valid TOML")
            .get("entry")
            .expect("the declaration holds an entry")
            .clone()
    }

    #[test]
    fn keeps_a_version_string_when_no_feature_is_added() {
        let rendered =
            render_inherited_dependency("anyhow", &workspace_entry("entry = \"1.0\"\n"), &[])
                .expect("the entry is inherited");

        assert_eq!(rendered.to_string(), " \"1.0\"");
    }

    #[test]
    fn keeps_a_table_entry_when_no_feature_is_added() {
        let rendered = render_inherited_dependency(
            "spiffe",
            &workspace_entry("entry = { version = \"0.6\", default-features = false }\n"),
            &[],
        )
        .expect("the entry is inherited");

        assert_eq!(
            rendered.to_string(),
            " { version = \"0.6\", default-features = false }"
        );
    }

    #[test]
    fn turns_a_version_string_into_a_table_when_a_feature_is_added() {
        let rendered = render_inherited_dependency(
            "serde",
            &workspace_entry("entry = \"1.0\"\n"),
            &["derive"],
        )
        .expect("the entry is inherited");

        assert_eq!(
            rendered.to_string(),
            " { version = \"1.0\", features = [\"derive\"] }"
        );
    }

    #[test]
    fn appends_to_the_features_the_workspace_already_declares() {
        let rendered = render_inherited_dependency(
            "chrono",
            &workspace_entry(
                "entry = { version = \"0.4\", default-features = false, features = [\"std\"] }\n",
            ),
            &["clock"],
        )
        .expect("the entry is inherited");

        assert_eq!(
            rendered.to_string(),
            " { version = \"0.4\", default-features = false, features = [\"std\", \"clock\"] }"
        );
    }

    #[test]
    fn reports_an_entry_that_is_not_a_value() {
        let error = render_inherited_dependency("broken", &workspace_entry("[entry]\n"), &[])
            .expect_err("a table entry is rejected");

        assert!(matches!(
            &error,
            ScaffoldError::WorkspaceDependencyMalformed { name } if name == "broken"
        ));
    }

    #[test]
    fn reports_an_entry_that_is_not_a_value_while_adding_features() {
        let error =
            render_inherited_dependency("broken", &workspace_entry("[entry]\n"), &["derive"])
                .expect_err("a table entry is rejected");

        assert!(matches!(
            &error,
            ScaffoldError::WorkspaceDependencyMalformed { name } if name == "broken"
        ));
    }

    #[test]
    fn reports_an_entry_that_is_neither_a_version_string_nor_a_table() {
        let error = render_inherited_dependency("broken", &workspace_entry("entry = 7\n"), &["x"])
            .expect_err("an integer entry is rejected");

        assert!(matches!(
            &error,
            ScaffoldError::WorkspaceDependencyMalformed { name } if name == "broken"
        ));
    }
}
