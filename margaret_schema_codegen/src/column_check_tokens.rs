use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_model::check_predicate::CheckPredicate;
use margaret_model_codegen::resolved_check::ResolvedCheck;

fn check_predicate_tokens(predicate: CheckPredicate) -> TokenStream {
    match predicate {
        CheckPredicate::ByteLength { length } => quote!(
            margaret::framework::model::check_predicate::CheckPredicate::ByteLength {
                length: #length,
            }
        ),
        CheckPredicate::Minimum { minimum } => quote!(
            margaret::framework::model::check_predicate::CheckPredicate::Minimum {
                minimum: #minimum,
            }
        ),
    }
}

pub(crate) fn column_check_tokens(check: &ResolvedCheck) -> TokenStream {
    let name = Literal::string(&check.name);
    let predicate = check_predicate_tokens(check.predicate);

    quote! {
        margaret::framework::model::column_check::ColumnCheck {
            name: #name,
            predicate: #predicate,
        }
    }
}
