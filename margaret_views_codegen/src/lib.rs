pub mod render_views;
pub mod views_artifacts;
pub mod views_codegen_error;
pub mod views_plan;

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

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::render_container::render_container;
    use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_tag_codegen::tag_pool::TagPool;

    use crate::render_views::render_views;
    use crate::views_artifacts::ViewsArtifacts;
    use crate::views_codegen_error::ViewsCodegenError;
    use crate::views_plan::ViewsPlan;

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

    fn bindings_for(index: &AttributeIndex) -> ContainerBindings {
        let registry = scan(index).expect("the console arguments are scanned");

        render_container(
            index,
            &registry,
            &[],
            &TagPool::collect(index).expect("the tags are collected"),
        )
        .expect("the container renders")
        .bindings
    }

    fn empty_bindings() -> ContainerBindings {
        bindings_for(&index_for(""))
    }

    fn generated(lib_source: &str) -> ViewsArtifacts {
        let index = index_for(lib_source);
        let bindings = bindings_for(&index);

        let plan = ViewsPlan::build(&index, &bindings).expect("the views are planned");

        render_views(plan, &bindings)
    }

    fn rejection_for(lib_source: &str) -> ViewsCodegenError {
        ViewsPlan::build(&index_for(lib_source), &empty_bindings())
            .map(drop)
            .expect_err("the invalid view is rejected")
    }

    fn rejection(lib_source: &str) -> String {
        rejection_for(lib_source).to_string()
    }

    #[test]
    fn rejects_a_view_absent_from_the_container_plan() {
        assert!(rejection(VALID_VIEW).contains("crate::CardLayout"));
    }

    fn formatted(modules: Vec<GeneratedModuleTokens>) -> String {
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
    fn generates_a_views_struct_and_builder() {
        let source = formatted(generated(VALID_VIEW).modules);

        assert!(source.contains("pub struct Views"));
        assert!(source.contains("pub the_card: ::std::sync::Arc<crate::CardLayout>"));
        assert!(source.contains("pub fn build("));
        assert!(source.contains("the_card: container.card_layout()"));
    }

    const CONSOLE_ARGUMENT_VIEW: &str = "\
#[renders_view(name = \"banner\")]
#[singleton]
struct Banner;

impl Banner {
    #[constructor]
    fn create(#[console_argument(from = \"title\")] title: String) -> anyhow::Result<Self> {}
}
";

    #[test]
    fn records_a_view_serve_input_and_reads_the_preconstructed_view() {
        let artifacts = generated(CONSOLE_ARGUMENT_VIEW);
        let source = formatted(artifacts.modules);
        let slot = artifacts.serve_inputs.len();

        assert_eq!(slot, 1);
        assert_eq!(artifacts.serve_inputs[0].name(), "title");
        assert!(source.contains("banner: container.banner()"));
        assert!(!source.contains("serve_input_"));
    }

    #[test]
    fn rejects_a_view_that_is_not_a_struct() {
        assert!(
            rejection("#[renders_view(name = \"bad\")]\n#[singleton]\nenum Bad {}\n")
                .contains("is not a struct")
        );
    }

    #[test]
    fn accepts_a_view_without_a_singleton() {
        let source =
            formatted(generated("#[renders_view(name = \"plain\")]\nstruct Plain;\n").modules);

        assert!(source.contains("pub plain: ::std::sync::Arc<crate::Plain>"));
        assert!(source.contains("plain: container.plain()"));
    }

    #[test]
    fn rejects_a_struct_declaring_more_than_one_view() {
        assert!(
            rejection(
                "#[renders_view(name = \"one\")]\n#[renders_view(name = \"two\")]\n#[singleton]\nstruct Bad;\n"
            )
            .contains("is declared more than once")
        );
    }

    #[test]
    fn rejects_a_view_without_a_name() {
        assert!(
            rejection("#[renders_view]\n#[singleton]\nstruct Bad;\n")
                .contains("is missing the 'name' argument")
        );
    }

    #[test]
    fn rejects_a_view_name_that_is_not_snake_case() {
        assert!(
            rejection("#[renders_view(name = \"CardLayout\")]\n#[singleton]\nstruct Bad;\n")
                .contains("must be a snake_case identifier")
        );
    }

    #[test]
    fn rejects_two_views_with_the_same_name() {
        assert!(
            rejection(
                "#[renders_view(name = \"card\")]\n#[singleton]\nstruct First;\n\n#[renders_view(name = \"card\")]\n#[singleton]\nstruct Second;\n"
            )
            .contains("each view name may identify at most one view")
        );
    }

    #[test]
    fn propagates_malformed_view_arguments() {
        assert!(
            rejection("#[renders_view(= 5)]\n#[singleton]\nstruct Bad;\n")
                .contains("failed to index the crate")
        );
    }

    #[test]
    fn propagates_a_non_string_view_name() {
        let error = rejection_for("#[renders_view(name = 5)]\n#[singleton]\nstruct Bad;\n");

        assert!(matches!(
            error,
            ViewsCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument {
                    ref key,
                    ref expected,
                    ..
                }
            } if key == "name" && expected == "string literal"
        ));
    }

    #[test]
    fn renders_a_non_fallible_empty_builder_when_the_container_has_no_accessors() {
        let source = formatted(generated("struct Plain;\n").modules);

        assert!(source.contains("pub struct Views {}"));
        assert!(source.contains(
            "pub fn build(container: &super::super::container::Container) -> super::Views {"
        ));
    }

    #[test]
    fn supports_same_struct_name_views_in_different_modules() {
        let source = formatted(
            generated(
                "mod a {\n#[renders_view(name = \"a_card\")]\n#[singleton]\nstruct Card;\n}\n\nmod b {\n#[renders_view(name = \"b_card\")]\n#[singleton]\nstruct Card;\n}\n",
            )
            .modules,
        );

        assert!(source.contains("pub a_card: ::std::sync::Arc<crate::a::Card>"));
        assert!(source.contains("pub b_card: ::std::sync::Arc<crate::b::Card>"));
    }
}
