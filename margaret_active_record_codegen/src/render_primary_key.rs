use proc_macro2::TokenStream;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::model::Model;

use crate::bound_read::BoundRead;
use crate::chained_construction::chained_construction;
use crate::chained_writes::chained_writes;
use crate::field_identifier::field_identifier;
use crate::field_type_tokens::field_type_tokens;
use crate::generated_model_module::generated_model_module;
use crate::key_targeting::KeyTargeting;
use crate::value_identifier::value_identifier;

pub(crate) fn render_primary_key(
    model: &Model,
    key_targeting: KeyTargeting,
) -> GeneratedModuleTokens {
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
    let reads: Vec<BoundRead> = key_fields
        .iter()
        .enumerate()
        .map(|(position, _field)| BoundRead {
            read: quote! { margaret::framework::active_record::value::Value::read(cursor) },
            value: value_identifier(position),
        })
        .collect();
    let assignments = key_fields
        .iter()
        .zip(&reads)
        .map(|(field, BoundRead { value, .. })| {
            let identifier = field_identifier(field);

            quote! { #identifier: #value }
        });
    let read = chained_construction(&reads, &quote! { Self { #(#assignments),* } });
    let writes: Vec<TokenStream> = key_fields
        .iter()
        .map(|field| {
            let identifier = field_identifier(field);

            quote! { margaret::framework::active_record::value::Value::write(&self.#identifier, parameters) }
        })
        .collect();
    let write = chained_writes(&writes);

    let value = match key_targeting {
        KeyTargeting::Referenced => quote! {
                impl margaret::framework::active_record::value::Value for PrimaryKey {
                    const WIDTH: usize = #(#widths)+*;

                    fn read(
                        cursor: &mut margaret::framework::active_record::row_cursor::RowCursor<'_>,
                    ) -> ::std::result::Result<
                        Self,
                        margaret::framework::active_record::active_record_error::ActiveRecordError,
                    > {
                        #read
                    }

                    fn write(
                        &self,
                        parameters: &mut margaret::framework::active_record::parameters::Parameters,
                    ) -> ::std::result::Result<
                        (),
                        margaret::framework::active_record::active_record_error::ActiveRecordError,
                    > {
                        #write
                    }
                }
        },
        KeyTargeting::Unreferenced => quote! {},
    };

    GeneratedModuleTokens::new(
        generated_model_module(model, "primary_key"),
        quote! {
            #[derive(Clone, Debug, PartialEq)]
            pub struct PrimaryKey {
                #(#declarations),*
            }

            #value
        },
    )
}
