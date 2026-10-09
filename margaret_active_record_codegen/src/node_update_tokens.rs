use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_model_codegen::model::Model;

use crate::assignable_fields::assignable_fields;

pub(crate) fn node_update_tokens(model: &Model, first: &Ident) -> TokenStream {
    if assignable_fields(model).is_empty() {
        return quote! {};
    }

    let record = path_tokens(&model.path);

    quote! {
        impl Step {
            #[doc = " # Errors"]
            #[doc = ""]
            #[doc = " Returns `ActiveRecordError` when the rows cannot be updated."]
            pub async fn update<Executing: margaret::framework::database::executor::Executor>(
                self,
                executor: &Executing,
                assign: impl FnOnce(
                    <#record as margaret::framework::active_record::assignable::Assignable>::Columns<
                        margaret::framework::active_record::assigning::Assigning,
                    >,
                ) -> margaret::framework::active_record::assigned::Assigned<
                    #record,
                    margaret::framework::active_record::assigning::Assigning,
                >,
            ) -> ::std::result::Result<
                u64,
                margaret::framework::active_record::active_record_error::ActiveRecordError,
            > {
                margaret::framework::active_record::prefix::Prefix::new(self.#first.into_narrowed())
                    .update(executor, assign)
                    .await
            }
        }
    }
}
