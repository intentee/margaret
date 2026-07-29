use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_model_codegen::model::Model;
use margaret_model_codegen::resolved_column::ResolvedColumn;
use margaret_model_codegen::resolved_foreign_key::ResolvedForeignKey;
use margaret_model_codegen::resolved_index::ResolvedIndex;

fn string_vec(values: &[String]) -> TokenStream {
    let items = values.iter().map(|value| {
        let value = Literal::string(value);

        quote! { #value.to_string() }
    });

    quote! { vec![#(#items),*] }
}

fn render_column(column: &ResolvedColumn) -> TokenStream {
    let column_type = &column.inferred.column_type;
    let default = &column.inferred.default;
    let nullable = column.inferred.nullable;
    let name = Literal::string(&column.name);

    quote! {
        margaret::framework::model::column::Column {
            column_type: #column_type,
            default: #default,
            name: #name.to_string(),
            nullable: #nullable,
        }
    }
}

fn render_foreign_key(foreign_key: &ResolvedForeignKey) -> TokenStream {
    let column = Literal::string(&foreign_key.column);
    let on_delete = &foreign_key.on_delete;
    let references_column = Literal::string(&foreign_key.references_column);
    let references_table = Literal::string(&foreign_key.references_table);

    quote! {
        margaret::framework::model::foreign_key::ForeignKey {
            column: #column.to_string(),
            on_delete: #on_delete,
            references_column: #references_column.to_string(),
            references_table: #references_table.to_string(),
        }
    }
}

fn render_index(index: &ResolvedIndex) -> TokenStream {
    let columns = string_vec(&index.columns);
    let name = Literal::string(&index.name);

    quote! {
        margaret::framework::model::index::Index {
            columns: #columns,
            name: #name.to_string(),
        }
    }
}

fn render_unique_constraint(columns: &[String]) -> TokenStream {
    let columns = string_vec(columns);

    quote! {
        margaret::framework::model::unique_constraint::UniqueConstraint {
            columns: #columns,
        }
    }
}

fn render_table(model: &Model) -> TokenStream {
    let table = Literal::string(&model.table);
    let columns = model.columns.iter().map(render_column);
    let foreign_keys = model.foreign_keys.iter().map(render_foreign_key);
    let indexes = model.indexes.iter().map(render_index);
    let primary_key = string_vec(&model.primary_key);
    let unique_constraints = model
        .unique_constraints
        .iter()
        .map(|unique_constraint| render_unique_constraint(&unique_constraint.columns));

    quote! {
        margaret::framework::model::table::Table {
            columns: vec![#(#columns),*],
            foreign_keys: vec![#(#foreign_keys),*],
            indexes: vec![#(#indexes),*],
            name: #table.to_string(),
            primary_key: #primary_key,
            unique_constraints: vec![#(#unique_constraints),*],
        }
    }
}

pub(crate) fn render(models: &[Model]) -> TokenStream {
    let tables = models.iter().map(render_table);
    let too_many_lines = too_many_lines_allow();

    quote! {
        #[must_use]
        #too_many_lines
        pub fn schema() -> margaret::framework::model::schema::Schema {
            margaret::framework::model::schema::Schema {
                tables: vec![#(#tables),*],
            }
        }
    }
}
