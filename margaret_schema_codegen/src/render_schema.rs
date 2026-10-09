use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

#[must_use]
pub fn render_schema(table_sets: &[CanonicalPath]) -> GeneratedModuleTokens {
    let table_sets = table_sets.iter().map(path_tokens);

    GeneratedModuleTokens::new(
        "schema",
        quote! {
            pub const SCHEMA: margaret::framework::model::schema::Schema =
                margaret::framework::model::schema::Schema {
                    table_sets: &[#(#table_sets),*],
                };
        },
    )
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use super::render_schema;

    #[test]
    fn lists_the_table_sets_in_the_given_order() {
        let schema = render_schema(&[
            CanonicalPath::new(
                [
                    "margaret",
                    "framework",
                    "grants",
                    "margaret",
                    "tables",
                    "TABLES",
                ]
                .map(ToString::to_string)
                .to_vec(),
            ),
            CanonicalPath::new(
                ["crate", "margaret", "tables", "TABLES"]
                    .map(ToString::to_string)
                    .to_vec(),
            ),
        ]);

        assert_eq!(schema.name(), "schema");
        assert_eq!(
            schema.to_source().split_whitespace().collect::<String>(),
            "pubconstSCHEMA:margaret::framework::model::schema::Schema=margaret::framework::model::schema::Schema{table_sets:&[::margaret::framework::grants::margaret::tables::TABLES,crate::margaret::tables::TABLES],};"
        );
    }
}
