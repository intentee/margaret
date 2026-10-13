use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;
use margaret_model_codegen::model_field::ModelField;
use margaret_schema_codegen::model_module_path::model_module_path;

use crate::generated_model_path::generated_model_path;
use crate::is_defaulted::is_defaulted;
use crate::node_location::NodeLocation;

pub(crate) fn render_model_impl(
    model: &Model,
    assignable: &[&ModelField],
) -> GeneratedModuleTokens {
    let record = path_tokens(&model.path);
    let generated = generated_model_path(model);
    let query = NodeLocation::query().step().path(model);
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
        model_module_path(model).child("model"),
        quote! {
            impl margaret::framework::active_record::model::Model for #record {
                type Conditions = #generated::conditions::Conditions;
                type Query = #query::Step;
            }

            #assignable_impl

            #creatable
        },
    )
}
