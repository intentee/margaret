use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;

use crate::console_argument::ConsoleArgument;
use crate::console_command::ConsoleCommand;

pub(crate) fn render(commands: &[ConsoleCommand], has_http: bool) -> String {
    let subcommands = commands.iter().map(subcommand_registration);
    let arms = commands.iter().map(command_arm);

    let serve_registration = if has_http {
        quote! {
            .subcommand(
                clap::Command::new("serve")
                    .arg(clap::Arg::new("addr").long("addr").required(true)),
            )
        }
    } else {
        quote! {}
    };

    let serve_arm = if has_http {
        quote! {
            Some(("serve", matches)) => {
                margaret_console::serve::serve(super::http::server(container), matches).await
            }
        }
    } else {
        quote! {}
    };

    let tokens = quote! {
        pub async fn run<Arguments, Argument>(
            container: &super::container::Container,
            args: Arguments,
        ) -> margaret_console::command_outcome::CommandOutcome
        where
            Arguments: IntoIterator<Item = Argument>,
            Argument: Into<std::ffi::OsString> + Clone,
        {
            let mut command = clap::Command::new(env!("CARGO_PKG_NAME"))
                #(#subcommands)*
                #serve_registration;

            match command.try_get_matches_from_mut(args) {
                Ok(matches) => match matches.subcommand() {
                    #(#arms)*
                    #serve_arm
                    _ => margaret_console::print_help::print_help(&mut command),
                },
                Err(error) => {
                    margaret_console::outcome_for_clap_error::outcome_for_clap_error(error)
                }
            }
        }
    };
    let file =
        syn::parse2::<syn::File>(tokens).expect("the generated tokens form a valid Rust file");

    prettyplease::unparse(&file)
}

fn subcommand_registration(command: &ConsoleCommand) -> TokenStream {
    let name = &command.name;
    let about = match &command.description {
        Some(description) => quote! { .about(#description) },
        None => quote! {},
    };
    let arguments = command.arguments.iter().map(argument_registration);

    quote! {
        .subcommand(clap::Command::new(#name)#about #(#arguments)*)
    }
}

fn argument_registration(argument: &ConsoleArgument) -> TokenStream {
    match argument {
        ConsoleArgument::Flag { name } => quote! {
            .arg(clap::Arg::new(#name).long(#name).action(clap::ArgAction::SetTrue))
        },
        ConsoleArgument::Named {
            name,
            required,
            value_type,
        } => quote! {
            .arg(
                clap::Arg::new(#name)
                    .long(#name)
                    .required(#required)
                    .value_parser(clap::value_parser!(#value_type)),
            )
        },
        ConsoleArgument::Positional {
            id,
            required,
            value_type,
        } => quote! {
            .arg(
                clap::Arg::new(#id)
                    .required(#required)
                    .value_parser(clap::value_parser!(#value_type)),
            )
        },
    }
}

fn command_arm(command: &ConsoleCommand) -> TokenStream {
    let name = &command.name;
    let accessor = &command.accessor;
    let matches_binding = if command.arguments.is_empty() {
        quote! { _matches }
    } else {
        quote! { matches }
    };
    let values = command.arguments.iter().map(argument_value);

    quote! {
        Some((#name, #matches_binding)) => container.#accessor().run(#(#values),*).await,
    }
}

fn argument_value(argument: &ConsoleArgument) -> TokenStream {
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

fn value_expression(id: &str, required: bool, value_type: &Type) -> TokenStream {
    if required {
        quote! {
            matches
                .get_one::<#value_type>(#id)
                .expect("a required console argument is present")
                .clone()
        }
    } else {
        quote! { matches.get_one::<#value_type>(#id).cloned() }
    }
}
