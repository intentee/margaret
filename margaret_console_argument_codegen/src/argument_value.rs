use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::console_argument::ConsoleArgument;
use crate::required_flag_read::required_flag_read;

fn value_expression(id: &str, required: bool, value_type: &Type) -> TokenStream {
    if required {
        required_flag_read(&quote! { #value_type }, id, &quote! { value.clone() })
    } else {
        quote! { matches.get_one::<#value_type>(#id).cloned() }
    }
}

pub fn argument_value(argument: &ConsoleArgument) -> TokenStream {
    match argument {
        ConsoleArgument::Flag { name } => quote! { matches.get_flag(#name) },
        ConsoleArgument::Named {
            name,
            required,
            value_type,
        } => value_expression(name, *required, value_type),
        ConsoleArgument::Positional {
            id,
            required,
            value_type,
        } => value_expression(id, *required, value_type),
    }
}
