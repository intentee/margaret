use crate::scaffolded_file::ScaffoldedFile;
use crate::scaffolded_module::ScaffoldedModule;

pub(crate) fn format_scaffolded_module(
    ScaffoldedModule {
        file,
        relative_path,
    }: ScaffoldedModule,
) -> ScaffoldedFile {
    ScaffoldedFile {
        contents: prettyplease::unparse(&file),
        relative_path,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use syn::parse_quote;

    use super::format_scaffolded_module;
    use crate::scaffolded_module::ScaffoldedModule;

    #[test]
    fn unparses_the_syntax_tree_into_the_scaffolded_source() {
        let formatted = format_scaffolded_module(ScaffoldedModule {
            file: parse_quote! {
                fn main() {}
            },
            relative_path: PathBuf::from("build.rs"),
        });

        assert_eq!(formatted.relative_path, PathBuf::from("build.rs"));
        assert_eq!(formatted.contents, "fn main() {}\n");
    }
}
