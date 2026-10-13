use proc_macro2::TokenStream;
use quote::quote;

use margaret_sql_identifier::table_namespace::TableNamespace;

pub(crate) fn table_namespace_tokens(namespace: TableNamespace) -> TokenStream {
    match namespace {
        TableNamespace::Application => {
            quote!(
                margaret::framework::sql_identifier::table_namespace::TableNamespace::Application
            )
        }
        TableNamespace::Framework => {
            quote!(margaret::framework::sql_identifier::table_namespace::TableNamespace::Framework)
        }
    }
}
