pub mod has_views;
pub mod render_views;
pub mod views_codegen_error;

mod render;
mod render_build;
mod view;
mod view_arguments;
mod views;

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;
    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;

    use crate::has_views::has_views;
    use crate::render_views::render_views;

    fn crate_with(lib_source: &str) -> TempDir {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        directory
    }

    fn index_for(lib_source: &str) -> AttributeIndex {
        let directory = crate_with(lib_source);

        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", directory.path().join("src")))
            .expect("the crate is indexed")
            .build()
    }

    fn formatted(
        modules: Vec<margaret_generated_module::generated_module_tokens::GeneratedModuleTokens>,
    ) -> String {
        modules
            .into_iter()
            .map(|module| {
                module
                    .format()
                    .expect("the generated module is a valid Rust file")
                    .source()
                    .to_string()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    const VALID_VIEW: &str = "\
#[renders_view(name = \"the_card\")]
#[singleton]
struct CardLayout;
";

    #[test]
    fn detects_the_presence_of_views() {
        assert!(has_views(&index_for(VALID_VIEW)));
        assert!(!has_views(&index_for("struct Plain;\n")));
    }

    #[test]
    fn generates_a_views_struct_and_builder() {
        let source =
            formatted(render_views(&index_for(VALID_VIEW)).expect("the views are generated"));

        assert!(source.contains("pub struct Views"));
        assert!(source.contains("pub the_card: ::std::sync::Arc<crate::CardLayout>"));
        assert!(source.contains("pub async fn build("));
        assert!(source.contains("the_card: container.card_layout().await"));
    }

    #[test]
    fn rejects_a_view_that_is_not_a_struct() {
        let message = render_views(&index_for(
            "#[renders_view(name = \"bad\")]\n#[singleton]\nenum Bad {}\n",
        ))
        .expect_err("a non-struct view is rejected")
        .to_string();

        assert!(message.contains("is not a struct"));
    }

    #[test]
    fn rejects_a_view_without_a_singleton() {
        let message = render_views(&index_for("#[renders_view(name = \"bad\")]\nstruct Bad;\n"))
            .expect_err("a view without #[singleton] is rejected")
            .to_string();

        assert!(message.contains("must also carry #[singleton]"));
    }

    #[test]
    fn rejects_a_view_that_provides_an_interface() {
        let message = render_views(&index_for(
            "#[renders_view(name = \"bad\")]\n#[singleton(provides = SomeTrait)]\nstruct Bad;\n",
        ))
        .expect_err("a view that provides an interface is rejected")
        .to_string();

        assert!(message.contains("renders as its concrete type"));
    }

    #[test]
    fn rejects_a_struct_declaring_more_than_one_view() {
        let message = render_views(&index_for(
            "#[renders_view(name = \"one\")]\n#[renders_view(name = \"two\")]\n#[singleton]\nstruct Bad;\n",
        ))
        .expect_err("a struct with two views is rejected")
        .to_string();

        assert!(message.contains("is declared more than once"));
    }

    #[test]
    fn rejects_a_view_without_a_name() {
        let message = render_views(&index_for("#[renders_view]\n#[singleton]\nstruct Bad;\n"))
            .expect_err("a view without a name is rejected")
            .to_string();

        assert!(message.contains("is missing the 'name' argument"));
    }

    #[test]
    fn rejects_a_view_name_that_is_not_snake_case() {
        let message = render_views(&index_for(
            "#[renders_view(name = \"CardLayout\")]\n#[singleton]\nstruct Bad;\n",
        ))
        .expect_err("a non-snake-case view name is rejected")
        .to_string();

        assert!(message.contains("must be a snake_case identifier"));
    }

    #[test]
    fn rejects_two_views_with_the_same_name() {
        let message = render_views(&index_for(
            "#[renders_view(name = \"card\")]\n#[singleton]\nstruct First;\n\n#[renders_view(name = \"card\")]\n#[singleton]\nstruct Second;\n",
        ))
        .expect_err("a duplicate view name is rejected")
        .to_string();

        assert!(message.contains("each view name may identify at most one view"));
    }

    #[test]
    fn propagates_malformed_view_arguments() {
        let message = render_views(&index_for(
            "#[renders_view(= 5)]\n#[singleton]\nstruct Bad;\n",
        ))
        .expect_err("malformed #[renders_view] arguments are rejected")
        .to_string();

        assert!(message.contains("failed to index the crate"));
    }

    #[test]
    fn propagates_a_non_string_view_name() {
        let message = render_views(&index_for(
            "#[renders_view(name = 5)]\n#[singleton]\nstruct Bad;\n",
        ))
        .expect_err("a non-string view name is rejected")
        .to_string();

        assert!(message.contains("failed to index the crate"));
    }

    #[test]
    fn propagates_malformed_singleton_arguments() {
        let message = render_views(&index_for(
            "#[renders_view(name = \"bad\")]\n#[singleton(= 5)]\nstruct Bad;\n",
        ))
        .expect_err("malformed #[singleton] arguments are rejected")
        .to_string();

        assert!(message.contains("failed to index the crate"));
    }

    #[test]
    fn propagates_a_non_path_singleton_provides() {
        let message = render_views(&index_for(
            "#[renders_view(name = \"bad\")]\n#[singleton(provides = \"s\")]\nstruct Bad;\n",
        ))
        .expect_err("a non-path #[singleton(provides)] is rejected")
        .to_string();

        assert!(message.contains("failed to index the crate"));
    }

    #[test]
    fn supports_same_struct_name_views_in_different_modules() {
        let source = formatted(
            render_views(&index_for(
                "mod a {\n#[renders_view(name = \"a_card\")]\n#[singleton]\nstruct Card;\n}\n\nmod b {\n#[renders_view(name = \"b_card\")]\n#[singleton]\nstruct Card;\n}\n",
            ))
            .expect("two Card views in different modules coexist"),
        );

        assert!(source.contains("pub a_card: ::std::sync::Arc<crate::a::Card>"));
        assert!(source.contains("pub b_card: ::std::sync::Arc<crate::b::Card>"));
    }
}
