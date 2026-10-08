use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;

use crate::field_identifier::field_identifier;
use crate::field_type_tokens::field_type_tokens;
use crate::generated_model_module::generated_model_module;

pub(crate) fn render_conditions(model: &Model) -> GeneratedModuleTokens {
    let record = path_tokens(&model.path);
    let declarations = model.fields.iter().map(|field| {
        let identifier = field_identifier(field);
        let field_type = field_type_tokens(field);

        quote! {
            pub #identifier: margaret::framework::active_record::operand::Operand<#record, #field_type>
        }
    });
    let constructions = model.fields.iter().map(|field| {
        let identifier = field_identifier(field);
        let start = field.start;

        quote! { #identifier: margaret::framework::active_record::operand::Operand::new(#start) }
    });

    GeneratedModuleTokens::new(
        generated_model_module(model, "conditions"),
        quote! {
            pub struct Conditions {
                #(#declarations),*
            }

            impl margaret::framework::active_record::field_set::FieldSet for Conditions {
                fn fields() -> Self {
                    Self {
                        #(#constructions),*
                    }
                }
            }
        },
    )
}
