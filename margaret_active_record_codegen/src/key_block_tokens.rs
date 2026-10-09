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
            margaret::framework::active_record::primary_key_width::primary_key_width::<#model>()
        };

        if loads_children {
            Self {
                read_base: quote! {
                    cursor.record::<#model>().inspect(|_| {
                        cursor.skip(#key_width);
                    })
                },
                select: quote! { builder.primary_key::<#model>(source); },
                width: quote! { + #key_width },
            }
        } else {
            Self {
                read_base: quote! { cursor.record::<#model>() },
                select: quote! {},
                width: quote! {},
            }
        }
    }
}
