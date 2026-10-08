use proc_macro2::TokenStream;
use quote::quote;

pub(crate) struct KeyBlockTokens {
    pub(crate) read_base: TokenStream,
    pub(crate) select: TokenStream,
    pub(crate) width: TokenStream,
}

impl KeyBlockTokens {
    pub(crate) fn of(model: &TokenStream, loads_children: bool) -> Self {
        let key_width = quote! {
            <<#model as margaret::framework::active_record::record::Record>::PrimaryKey
                as margaret::framework::active_record::value::Value>::WIDTH
        };

        if loads_children {
            Self {
                read_base: quote! {
                    {
                        let base = cursor.record::<#model>()?;

                        cursor.skip(#key_width);
                        base
                    }
                },
                select: quote! { builder.primary_key::<#model>(source); },
                width: quote! { + #key_width },
            }
        } else {
            Self {
                read_base: quote! { cursor.record::<#model>()? },
                select: quote! {},
                width: quote! {},
            }
        }
    }
}
