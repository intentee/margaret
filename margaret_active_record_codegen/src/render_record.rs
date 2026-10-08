use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;
use margaret_schema_codegen::table_module_name::TABLE_MODULE_NAME;

use crate::field_identifier::field_identifier;
use crate::field_span_tokens::field_span_tokens;
use crate::generated_model_module::generated_model_module;
use crate::generated_model_path::generated_model_path;
use crate::primary_key_type_tokens::primary_key_type_tokens;

pub(crate) fn render_record(model: &Model) -> GeneratedModuleTokens {
    let record = path_tokens(&model.path);
    let generated = generated_model_path(model);
    let table = format_ident!("{TABLE_MODULE_NAME}");
    let primary_key_type = primary_key_type_tokens(model);
    let primary_key = if let [single] = model.indexes.primary_key.fields.as_slice() {
        let identifier = field_identifier(single);

        quote! { ::std::clone::Clone::clone(&self.#identifier) }
    } else {
        let identifiers = model
            .indexes
            .primary_key
            .fields
            .iter()
            .map(field_identifier);

        quote! {
            #generated::primary_key::PrimaryKey {
                #(#identifiers: ::std::clone::Clone::clone(&self.#identifiers)),*
            }
        }
    };
    let primary_key_spans = model
        .indexes
        .primary_key
        .fields
        .iter()
        .map(field_span_tokens);
    let reads = model.fields.iter().map(|field| {
        let identifier = field_identifier(field);

        quote! { #identifier: margaret::framework::active_record::field::Field::read(cursor)? }
    });
    let writes = model.fields.iter().map(|field| {
        let identifier = field_identifier(field);

        quote! { margaret::framework::active_record::field::Field::write(&self.#identifier, parameters)?; }
    });

    GeneratedModuleTokens::new(
        generated_model_module(model, "record"),
        quote! {
            impl margaret::framework::active_record::record::Record for #record {
                type PrimaryKey = #primary_key_type;

                const PRIMARY_KEY: &'static [margaret::framework::active_record::field_span::FieldSpan] =
                    &[#(#primary_key_spans),*];

                const TABLE: &'static margaret::framework::model::table::Table =
                    &#generated::#table::TABLE;

                fn primary_key(&self) -> Self::PrimaryKey {
                    #primary_key
                }

                fn read(
                    cursor: &mut margaret::framework::active_record::row_cursor::RowCursor<'_>,
                ) -> ::std::result::Result<
                    Self,
                    margaret::framework::active_record::active_record_error::ActiveRecordError,
                > {
                    ::std::result::Result::Ok(Self { #(#reads),* })
                }

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
