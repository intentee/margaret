use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;

use crate::models_module_name::MODELS_MODULE_NAME;
use crate::table_module_name::TABLE_MODULE_NAME;

#[must_use]
pub fn render_tables(models: &[Model]) -> GeneratedModuleTokens {
    let models_module = format_ident!("{MODELS_MODULE_NAME}");
    let table_module = format_ident!("{TABLE_MODULE_NAME}");
    let tables = models.iter().map(|model| {
        let module = format_ident!("{}", model.module);

        quote! { &crate::margaret::#models_module::#module::#table_module::TABLE }
    });

    GeneratedModuleTokens::new(
        "tables",
        quote! {
            pub const TABLES: &[&margaret::framework::model::table::Table] = &[#(#tables),*];
        },
    )
}

#[cfg(test)]
mod tests {
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_model_codegen::models::models;
    use margaret_sql_identifier::table_namespace::TableNamespace;

    use super::render_tables;

    #[test]
    fn lists_the_tables_in_foreign_key_order() {
        let indexed = IndexedSource::new(
            "use margaret::framework::active_record::key::Key;\n\n#[model(table = \"articles\")]\nstruct Article {\n    #[column(primary_key)]\n    id: i64,\n    #[column]\n    #[index]\n    author: Key<Author>,\n}\n\n#[model(table = \"authors\")]\nstruct Author {\n    #[column(primary_key)]\n    id: i64,\n}\n",
        );
        let tables = render_tables(
            &models(&indexed.index, TableNamespace::Application).expect("the models resolve"),
        );

        assert_eq!(tables.name(), "tables");
        assert_eq!(
            tables.to_source().split_whitespace().collect::<String>(),
            "pubconstTABLES:&[&margaret::framework::model::table::Table]=&[&crate::margaret::models::author::table::TABLE,&crate::margaret::models::article::table::TABLE];"
        );
    }
}
