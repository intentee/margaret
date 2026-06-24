use proc_macro2::TokenStream;
use quote::quote;

use crate::console_command::ConsoleCommand;

pub(crate) fn render(commands: &[ConsoleCommand], has_http: bool) -> String {
    let subcommands = commands.iter().map(subcommand_registration);
    let arms = commands.iter().map(command_arm);

    let command_trait_import = if commands.is_empty() {
        quote! {}
    } else {
        quote! { use margaret_console::command::Command; }
    };

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
        #command_trait_import

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
    let field = &command.field;
    let about = match &command.description {
        Some(description) => quote! { .about(#description) },
        None => quote! {},
    };

    quote! {
        .subcommand(clap::Command::new(#name)#about.args(container.#field.arguments()))
    }
}

fn command_arm(command: &ConsoleCommand) -> TokenStream {
    let name = &command.name;
    let field = &command.field;

    quote! {
        Some((#name, matches)) => container.#field.run(matches).await,
    }
}
