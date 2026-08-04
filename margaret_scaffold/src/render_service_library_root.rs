use std::path::PathBuf;

use syn::parse_quote;

use crate::module_declaration_tokens::module_declaration_tokens;
use crate::scaffolded_module::ScaffoldedModule;

pub(crate) fn render_service_library_root(module_names: &[&str]) -> ScaffoldedModule {
    let declarations = module_declaration_tokens(module_names);

    ScaffoldedModule {
        file: parse_quote! {
            #declarations

            #[rustfmt::skip]
            #[path = "../margaret/mod.rs"]
            pub mod margaret;
        },
        relative_path: PathBuf::from("src").join("lib.rs"),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::render_service_library_root;
    use crate::format_scaffolded_module::format_scaffolded_module;

    #[test]
    fn mounts_the_generated_umbrella_module_next_to_the_hand_written_ones() {
        let rendered = format_scaffolded_module(render_service_library_root(&["routes"]));

        assert_eq!(rendered.relative_path, PathBuf::from("src").join("lib.rs"));
        assert_eq!(
            rendered.contents,
            concat!(
                "pub mod routes;\n",
                "#[rustfmt::skip]\n",
                "#[path = \"../margaret/mod.rs\"]\n",
                "pub mod margaret;\n",
            )
        );
    }
}
