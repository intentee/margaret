use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;

use crate::field_identifier::field_identifier;
use crate::field_type_tokens::field_type_tokens;
use crate::generated_model_module::generated_model_module;
use crate::is_defaulted::is_defaulted;

pub(crate) fn render_draft(model: &Model) -> GeneratedModuleTokens {
    let drafted: Vec<_> = model
        .fields
        .iter()
        .filter(|field| !is_defaulted(field))
        .collect();
    let declarations = drafted.iter().map(|field| {
        let identifier = field_identifier(field);
        let field_type = field_type_tokens(field);

        quote! { pub #identifier: #field_type }
    });
    let writes = drafted.iter().map(|field| {
        let identifier = field_identifier(field);

        quote! { margaret::framework::active_record::field::Field::write(&self.#identifier, parameters)?; }
    });

    GeneratedModuleTokens::new(
        generated_model_module(model, "draft"),
        quote! {
            pub struct Draft {
                #(#declarations),*
            }

            impl margaret::framework::active_record::draft_record::DraftRecord for Draft {
                fn write(
                    &self,
                    parameters: &mut margaret::framework::active_record::parameters::Parameters,
                ) -> ::std::result::Result<
                    (),
                    margaret::framework::active_record::active_record_error::ActiveRecordError,
                > {
                    #(#writes)*

                    ::std::result::Result::Ok(())
                }
            }
        },
    )
}
