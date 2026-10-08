use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;
use margaret_model_codegen::model_field::ModelField;

use crate::field_identifier::field_identifier;
use crate::field_type_tokens::field_type_tokens;
use crate::generated_model_module::generated_model_module;

pub(crate) fn render_columns(model: &Model, assignable: &[&ModelField]) -> GeneratedModuleTokens {
    let record = path_tokens(&model.path);
    let declarations = assignable.iter().map(|field| {
        let identifier = field_identifier(field);
        let field_type = field_type_tokens(field);

        quote! {
            pub #identifier: margaret::framework::active_record::column::Column<#record, #field_type, Context>
        }
    });
    let constructions = assignable.iter().map(|field| {
        let identifier = field_identifier(field);
        let start = field.start;

        quote! { #identifier: margaret::framework::active_record::column::Column::new(#start) }
    });

    GeneratedModuleTokens::new(
        generated_model_module(model, "columns"),
        quote! {
            pub struct Columns<Context> {
                #(#declarations),*
            }

            impl<Context> margaret::framework::active_record::field_set::FieldSet for Columns<Context> {
                fn fields() -> Self {
                    Self {
                        #(#constructions),*
                    }
                }
            }
        },
    )
}
