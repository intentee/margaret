use proc_macro2::TokenStream;
use quote::quote;

use margaret_model::on_delete::OnDelete;

pub(crate) fn on_delete_tokens(on_delete: OnDelete) -> TokenStream {
    match on_delete {
        OnDelete::Cascade => quote!(margaret::framework::model::on_delete::OnDelete::Cascade),
        OnDelete::NoAction => quote!(margaret::framework::model::on_delete::OnDelete::NoAction),
        OnDelete::Restrict => quote!(margaret::framework::model::on_delete::OnDelete::Restrict),
        OnDelete::SetDefault => quote!(margaret::framework::model::on_delete::OnDelete::SetDefault),
        OnDelete::SetNull => quote!(margaret::framework::model::on_delete::OnDelete::SetNull),
    }
}
