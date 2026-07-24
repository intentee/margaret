use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_model::column_default::ColumnDefault;
use margaret_model::column_type::ColumnType;
use margaret_model::on_delete::OnDelete;
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

fn column_type_tokens(column_type: ColumnType) -> TokenStream {
    match column_type {
        ColumnType::BigInt => quote!(margaret_model::column_type::ColumnType::BigInt),
        ColumnType::Boolean => quote!(margaret_model::column_type::ColumnType::Boolean),
        ColumnType::Bytea => quote!(margaret_model::column_type::ColumnType::Bytea),
        ColumnType::Integer => quote!(margaret_model::column_type::ColumnType::Integer),
        ColumnType::Text => quote!(margaret_model::column_type::ColumnType::Text),
        ColumnType::Timestamptz => quote!(margaret_model::column_type::ColumnType::Timestamptz),
        ColumnType::Uuid => quote!(margaret_model::column_type::ColumnType::Uuid),
    }
}

fn column_default_tokens(default: ColumnDefault) -> TokenStream {
    match default {
        ColumnDefault::NotSet => quote!(margaret_model::column_default::ColumnDefault::NotSet),
        ColumnDefault::UuidV7 => quote!(margaret_model::column_default::ColumnDefault::UuidV7),
    }
}

fn on_delete_tokens(on_delete: OnDelete) -> TokenStream {
    match on_delete {
        OnDelete::Cascade => quote!(margaret_model::on_delete::OnDelete::Cascade),
        OnDelete::NoAction => quote!(margaret_model::on_delete::OnDelete::NoAction),
        OnDelete::Restrict => quote!(margaret_model::on_delete::OnDelete::Restrict),
        OnDelete::SetDefault => quote!(margaret_model::on_delete::OnDelete::SetDefault),
        OnDelete::SetNull => quote!(margaret_model::on_delete::OnDelete::SetNull),
    }
}

fn render_column(column: &ResolvedColumn) -> TokenStream {
    let column_type = column_type_tokens(column.inferred.column_type);
    let default = column_default_tokens(column.inferred.default);
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
    let columns = string_vec(&foreign_key.columns);
    let name = Literal::string(foreign_key.name.as_str());
    let on_delete = on_delete_tokens(foreign_key.on_delete);
    let references_columns = string_vec(&foreign_key.references_columns);
    let references_table = Literal::string(&foreign_key.references_table);

    quote! {
        margaret_model::foreign_key::ForeignKey {
            columns: #columns,
            name: #name.to_string(),
            on_delete: #on_delete,
            references_columns: #references_columns,
            references_table: #references_table.to_string(),
        }
    }
}

fn render_index(index: &ResolvedIndex) -> TokenStream {
    let columns = string_vec(&index.columns);
    let name = Literal::string(index.name.as_str());

    quote! {
        margaret_model::index::Index {
            columns: #columns,
            name: #name.to_string(),
        }
    }
}

fn render_unique_constraint(columns: &[String]) -> TokenStream {
    let columns = string_vec(columns);

    quote! {
        margaret_model::unique_constraint::UniqueConstraint {
            columns: #columns,
        }
    }
}

fn render_table(model: &Model) -> TokenStream {
    let table = Literal::string(model.table.as_str());
    let columns = model.columns.iter().map(render_column);
    let foreign_keys = model.foreign_keys.iter().map(render_foreign_key);
    let indexes = model.indexes.iter().map(render_index);
    let primary_key = string_vec(&model.primary_key);
    let unique_constraints = model
        .unique_constraints
        .iter()
        .map(|unique_constraint| render_unique_constraint(&unique_constraint.columns));

    quote! {
        margaret_model::table::Table {
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

    quote! {
        #[must_use]
        pub fn schema() -> margaret_model::schema::Schema {
            margaret_model::schema::Schema {
                tables: vec![#(#tables),*],
            }
        }
    }
}
