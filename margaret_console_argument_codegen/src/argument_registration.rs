use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::argument_relation::ArgumentRelation;
use crate::console_argument::ConsoleArgument;

fn argument_relation_call(relation: &ArgumentRelation) -> TokenStream {
    match relation {
        ArgumentRelation::RequiredIfEq { argument, value } => quote! {
            .required_if_eq(#argument, #value)
        },
    }
}

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
            relations,
            ..
        } => {
            let value_type = path_tokens(value_type);
            let relation_calls = relations.iter().map(argument_relation_call);

            quote! {
                .arg(
                    clap::Arg::new(#name)
                        .long(#name)
                        .required(#required)
                        .value_parser(clap::value_parser!(#value_type))
                        #(#relation_calls)*
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
    }
}

#[cfg(test)]
mod tests {
    use margaret_attributes::canonical_path::CanonicalPath;

    use crate::argument_relation::ArgumentRelation;
    use crate::console_argument::ConsoleArgument;
    use crate::weaving_kind::WeavingKind;

    use super::argument_registration;

    fn collapsed(argument: &ConsoleArgument) -> String {
        argument_registration(argument)
            .to_string()
            .split_whitespace()
            .collect()
    }

    #[test]
    fn a_named_argument_emits_its_required_if_eq_relation() {
        let named = ConsoleArgument::Named {
            name: "jwks-secret-file".to_string(),
            required: false,
            weaving: WeavingKind::Cloned,
            value_type: CanonicalPath::new(vec![
                "std".to_string(),
                "path".to_string(),
                "PathBuf".to_string(),
            ]),
            relations: vec![ArgumentRelation::RequiredIfEq {
                argument: "jwks-secret-storage".to_string(),
                value: "file".to_string(),
            }],
        };

        assert!(collapsed(&named).contains(r#".required_if_eq("jwks-secret-storage","file")"#));
    }
}
