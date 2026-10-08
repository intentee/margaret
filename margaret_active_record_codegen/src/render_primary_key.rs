use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;

use crate::field_identifier::field_identifier;
use crate::field_type_tokens::field_type_tokens;
use crate::generated_model_module::generated_model_module;

pub(crate) fn render_primary_key(model: &Model) -> GeneratedModuleTokens {
    let key_fields = &model.indexes.primary_key.fields;
    let declarations = key_fields.iter().map(|field| {
        let identifier = field_identifier(field);
        let field_type = field_type_tokens(field);

        quote! { pub #identifier: #field_type }
    });
    let widths = key_fields.iter().map(|field| {
        let field_type = field_type_tokens(field);

        quote! { <#field_type as margaret::framework::active_record::value::Value>::WIDTH }
    });
    let reads = key_fields.iter().map(|field| {
        let identifier = field_identifier(field);

        quote! { #identifier: margaret::framework::active_record::value::Value::read(cursor)? }
    });
    let writes = key_fields.iter().map(|field| {
        let identifier = field_identifier(field);

        quote! { margaret::framework::active_record::value::Value::write(&self.#identifier, parameters)?; }
    });

    GeneratedModuleTokens::new(
        generated_model_module(model, "primary_key"),
        quote! {
            #[derive(Clone, Debug, PartialEq)]
            pub struct PrimaryKey {
                #(#declarations),*
            }

            impl margaret::framework::active_record::value::Value for PrimaryKey {
                const WIDTH: usize = #(#widths)+*;

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
