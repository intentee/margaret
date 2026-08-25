use proc_macro2::TokenStream;
use quote::quote;

use margaret_model::column_type::ColumnType;

pub(crate) fn column_type_tokens(column_type: ColumnType) -> TokenStream {
    match column_type {
        ColumnType::BigInt => quote!(margaret::framework::model::column_type::ColumnType::BigInt),
        ColumnType::Boolean => quote!(margaret::framework::model::column_type::ColumnType::Boolean),
        ColumnType::Bytea => quote!(margaret::framework::model::column_type::ColumnType::Bytea),
        ColumnType::DoublePrecision => {
            quote!(margaret::framework::model::column_type::ColumnType::DoublePrecision)
        }
        ColumnType::Integer => quote!(margaret::framework::model::column_type::ColumnType::Integer),
        ColumnType::Numeric { precision, scale } => quote!(
            margaret::framework::model::column_type::ColumnType::Numeric {
                precision: #precision,
                scale: #scale,
            }
        ),
        ColumnType::Real => quote!(margaret::framework::model::column_type::ColumnType::Real),
        ColumnType::Text => quote!(margaret::framework::model::column_type::ColumnType::Text),
        ColumnType::Timestamptz => {
            quote!(margaret::framework::model::column_type::ColumnType::Timestamptz)
        }
        ColumnType::Uuid => quote!(margaret::framework::model::column_type::ColumnType::Uuid),
    }
}
