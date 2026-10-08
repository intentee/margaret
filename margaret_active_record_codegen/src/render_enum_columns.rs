use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_codegen_tokens::path_tokens::path_tokens;

pub(crate) fn render_enum_column(path: &CanonicalPath, variants: &[String]) -> TokenStream {
    let enum_type = path_tokens(path);
    let enum_name = Literal::string(&path.to_string());
    let identifiers: Vec<_> = variants
        .iter()
        .map(|variant| format_ident!("{variant}"))
        .collect();
    let names: Vec<Literal> = variants
        .iter()
        .map(|variant| Literal::string(variant))
        .collect();

    quote! {
        impl margaret::framework::active_record::enum_column::EnumColumn for #enum_type {
            const ENUM_TYPE: &'static str = #enum_name;

            fn from_variant_name(stored: &str) -> ::std::option::Option<Self> {
                match stored {
                    #(#names => ::std::option::Option::Some(Self::#identifiers),)*
                    _ => ::std::option::Option::None,
                }
            }

            fn variant_name(&self) -> &'static str {
                match self {
                    #(Self::#identifiers => #names,)*
                }
            }
        }
    }
}
