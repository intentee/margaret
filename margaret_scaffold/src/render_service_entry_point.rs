use std::path::PathBuf;

use proc_macro2::Ident;
use proc_macro2::Span;
use syn::parse_quote;

use crate::scaffolded_module::ScaffoldedModule;

pub(crate) fn render_service_entry_point(crate_name: &str) -> ScaffoldedModule {
    let service_crate = Ident::new(crate_name, Span::call_site());

    ScaffoldedModule {
        file: parse_quote! {
            use margaret::framework::console::command_outcome::CommandOutcome;

            #[tokio::main]
            async fn main() -> CommandOutcome {
                #service_crate::margaret::run::run(std::env::args_os()).await
            }
        },
        relative_path: PathBuf::from("src").join("main.rs"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::render_service_entry_point;
    use crate::format_scaffolded_module::format_scaffolded_module;

    #[test]
    fn hands_the_process_arguments_to_the_generated_console() {
        let rendered = format_scaffolded_module(render_service_entry_point("acme_identity"));

        assert_eq!(rendered.relative_path, PathBuf::from("src").join("main.rs"));
        assert_eq!(
            rendered.contents,
            concat!(
                "use margaret::framework::console::command_outcome::CommandOutcome;\n",
                "#[tokio::main]\n",
                "async fn main() -> CommandOutcome {\n",
                "    acme_identity::margaret::run::run(std::env::args_os()).await\n",
                "}\n",
            )
        );
    }
}
