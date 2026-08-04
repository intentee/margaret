use std::path::PathBuf;

use syn::parse_quote;

use crate::module_declaration_tokens::module_declaration_tokens;
use crate::scaffolded_module::ScaffoldedModule;

pub(crate) fn render_module_declarations(
    relative_path: PathBuf,
    module_names: &[&str],
) -> ScaffoldedModule {
    let declarations = module_declaration_tokens(module_names);

    ScaffoldedModule {
        file: parse_quote! { #declarations },
        relative_path,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::render_module_declarations;
    use crate::format_scaffolded_module::format_scaffolded_module;

    #[test]
    fn renders_a_module_root_that_only_declares_its_submodules() {
        let rendered = format_scaffolded_module(render_module_declarations(
            PathBuf::from("src").join("routes").join("mod.rs"),
            &["get_identity", "get_well_known_jwks"],
        ));

        assert_eq!(
            rendered.relative_path,
            PathBuf::from("src").join("routes").join("mod.rs")
        );
        assert_eq!(
            rendered.contents,
            "pub mod get_identity;\npub mod get_well_known_jwks;\n"
        );
    }
}
