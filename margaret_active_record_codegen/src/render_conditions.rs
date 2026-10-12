use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;
use margaret_schema_codegen::model_module_path::model_module_path;

use crate::field_identifier::field_identifier;
use crate::field_type_tokens::field_type_tokens;

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

        quote! {
            #identifier: margaret::framework::active_record::operand::Operand {
                field: ::std::marker::PhantomData,
                start: #start,
            }
        }
    });

    GeneratedModuleTokens::new(
        model_module_path(model).child("conditions"),
        quote! {
            pub struct Conditions {
                #(#declarations),*
            }

            impl margaret::framework::active_record::field_set::FieldSet for Conditions {
                const FIELDS: Self = Self {
                    #(#constructions),*
                };
            }
        },
    )
}
