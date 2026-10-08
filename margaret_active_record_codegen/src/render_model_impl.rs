use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;
use margaret_model_codegen::model_field::ModelField;

use crate::generated_model_module::generated_model_module;
use crate::generated_model_path::generated_model_path;
use crate::is_defaulted::is_defaulted;

pub(crate) fn render_model_impl(
    model: &Model,
    assignable: &[&ModelField],
) -> GeneratedModuleTokens {
    let record = path_tokens(&model.path);
    let generated = generated_model_path(model);
    let assignable_impl = if assignable.is_empty() {
        quote! {}
    } else {
        quote! {
            impl margaret::framework::active_record::assignable::Assignable for #record {
                type Columns<Context> = #generated::columns::Columns<Context>;
            }
        }
    };
    let creatable = if model.fields.iter().any(is_defaulted) {
        quote! {
            impl margaret::framework::active_record::creatable::Creatable for #record {
                type Draft = #generated::draft::Draft;
            }
        }
    } else {
        quote! {}
    };

    GeneratedModuleTokens::new(
        generated_model_module(model, "model"),
        quote! {
            impl margaret::framework::active_record::model::Model for #record {
                type Conditions = #generated::conditions::Conditions;
                type Query = #generated::query::Step;
            }

            #assignable_impl

            #creatable
        },
    )
}
