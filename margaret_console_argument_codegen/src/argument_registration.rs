use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::console_argument::ConsoleArgument;

#[must_use]
pub fn argument_registration(argument: &ConsoleArgument) -> TokenStream {
    match argument {
        ConsoleArgument::Flag { name } => quote! {
            .arg(clap::Arg::new(#name).long(#name).action(clap::ArgAction::SetTrue))
        },
        ConsoleArgument::Named {
            name,
            required,
            value_type,
            ..
        } => {
            let value_type = path_tokens(value_type);

            quote! {
                .arg(
                    clap::Arg::new(#name)
                        .long(#name)
                        .required(#required)
                        .value_parser(clap::value_parser!(#value_type)),
                )
            }
        }
        ConsoleArgument::Positional {
            id,
            required,
            value_type,
            ..
        } => {
            let value_type = path_tokens(value_type);

            quote! {
                .arg(
                    clap::Arg::new(#id)
                        .required(#required)
                        .value_parser(clap::value_parser!(#value_type)),
                )
            }
        }
        ConsoleArgument::SpiffeHttpClient => quote! {},
    }
}
