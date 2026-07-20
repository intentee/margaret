use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;

use crate::model::Model;
use crate::resolved_column::ResolvedColumn;
use crate::resolved_foreign_key::ResolvedForeignKey;

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

fn render_foreign_key(foreign_key: &ResolvedForeignKey) -> TokenStream {
    let columns = foreign_key.columns.iter().map(|column| {
        let column = Literal::string(column);

        quote! { #column.to_string() }
    });
    let references_columns = foreign_key.references_columns.iter().map(|column| {
        let column = Literal::string(column);

        quote! { #column.to_string() }
    });
    let references_table = Literal::string(&foreign_key.references_table);

    quote! {
        margaret_model::foreign_key::ForeignKey {
            columns: vec![#(#columns),*],
            references_columns: vec![#(#references_columns),*],
            references_table: #references_table.to_string(),
        }
    }
}

fn render_table(model: &Model) -> TokenStream {
    let table = Literal::string(&model.table);
    let columns = model.columns.iter().map(render_column);
    let foreign_keys = model.foreign_keys.iter().map(render_foreign_key);
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
            foreign_keys: vec![#(#foreign_keys),*],
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
