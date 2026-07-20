use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;

use crate::model::Model;
use crate::resolved_column::ResolvedColumn;

fn render_column(column: &ResolvedColumn) -> TokenStream {
    let column_type = &column.inferred.column_type;
    let default = &column.inferred.default;
    let nullable = column.inferred.nullable;
    let name = Literal::string(&column.name);

    quote! {
        margaret_model::column::Column {
            column_type: #column_type,
            default: #default,
            name: #name.to_string(),
            nullable: #nullable,
        }
    }
}

fn render_table(model: &Model) -> TokenStream {
    let table = Literal::string(&model.table);
    let columns = model.columns.iter().map(render_column);
    let primary_key = model
        .columns
        .iter()
        .filter(|column| column.primary_key)
        .map(|column| {
            let name = Literal::string(&column.name);

            quote! { #name.to_string() }
        });

    quote! {
        margaret_model::table::Table {
            columns: vec![#(#columns),*],
            name: #table.to_string(),
            primary_key: vec![#(#primary_key),*],
        }
    }
}

pub(crate) fn render(models: &[Model]) -> TokenStream {
    let tables = models.iter().map(render_table);

    quote! {
        #[must_use]
        pub fn schema() -> margaret_console::command_outcome::CommandOutcome {
            let schema = margaret_model::schema::Schema {
                tables: vec![#(#tables),*],
            };

            println!(
                "{}",
                margaret_model::render_postgres::render_postgres(&schema)
            );

            margaret_console::command_outcome::CommandOutcome::Succeeded
        }
    }
}
