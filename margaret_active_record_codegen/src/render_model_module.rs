use std::collections::HashSet;

use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::field_value::FieldValue;
use margaret_model_codegen::model::Model;
use margaret_schema_codegen::models_module_name::MODELS_MODULE_NAME;
use margaret_schema_codegen::table_module_name::TABLE_MODULE_NAME;

use crate::assignable_fields::assignable_fields;
use crate::generated_model_module::generated_model_module;
use crate::is_defaulted::is_defaulted;
use crate::key_targeting::KeyTargeting;
use crate::render_columns::render_columns;
use crate::render_conditions::render_conditions;
use crate::render_draft::render_draft;
use crate::render_enum_columns::render_enum_column;
use crate::render_model_impl::render_model_impl;
use crate::render_primary_key::render_primary_key;
use crate::render_query::render_query;
use crate::render_record::render_record;

pub(crate) fn render_model_module(
    model: &Model,
    key_targets: &HashSet<CanonicalPath>,
    rendered_enums: &mut HashSet<CanonicalPath>,
) -> Vec<GeneratedModuleTokens> {
    let key_targeting = if key_targets.contains(&model.path) {
        KeyTargeting::Referenced
    } else {
        KeyTargeting::Unreferenced
    };
    let assignable = assignable_fields(model);
    let table = format_ident!("{TABLE_MODULE_NAME}");
    let mut declarations = Vec::new();
    let mut modules = vec![
        render_conditions(model),
        render_model_impl(model, &assignable),
    ];

    if !assignable.is_empty() {
        declarations.push(quote! { pub mod columns; });
        modules.push(render_columns(model, &assignable));
    }

    declarations.push(quote! { pub mod conditions; });

    if model.fields.iter().any(is_defaulted) {
        declarations.push(quote! { pub mod draft; });
        modules.push(render_draft(model));
    }

    let enum_columns: Vec<_> = model
        .fields
        .iter()
        .filter_map(|field| match &field.value {
            FieldValue::Enum { path, variants } if rendered_enums.insert(path.clone()) => {
                Some(render_enum_column(path, variants))
            }
            FieldValue::Enum { .. }
            | FieldValue::Json { .. }
            | FieldValue::Key { .. }
            | FieldValue::Scalar { .. } => None,
        })
        .collect();

    if !enum_columns.is_empty() {
        declarations.push(quote! { mod enum_columns; });
        modules.push(GeneratedModuleTokens::new(
            generated_model_module(model, "enum_columns"),
            quote! { #(#enum_columns)* },
        ));
    }

    declarations.push(quote! { mod model; });

    if model.indexes.primary_key.fields.len() > 1 {
        declarations.push(quote! { pub mod primary_key; });
        modules.push(render_primary_key(model, key_targeting));
    }

    declarations.push(quote! { pub mod query; });
    declarations.push(quote! { mod record; });
    declarations.push(quote! { pub mod #table; });
    modules.push(render_record(model, key_targeting));
    modules.extend(render_query(model));
    modules.push(GeneratedModuleTokens::new(
        format!("{MODELS_MODULE_NAME}/{}", model.module),
        quote! { #(#declarations)* },
    ));

    modules
}
