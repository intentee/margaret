use quote::format_ident;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::render_shape::render_shape;
use crate::shape_declaration::ShapeDeclaration;
use crate::shapes_module_name::SHAPES_MODULE_NAME;

#[must_use]
pub fn render_shapes(shapes: &[ShapeDeclaration]) -> Vec<GeneratedModuleTokens> {
    if shapes.is_empty() {
        return Vec::new();
    }

    let declarations = shapes.iter().map(|shape| {
        let module = format_ident!("{}", shape.module);

        quote! { pub mod #module; }
    });

    [GeneratedModuleTokens::new(
        SHAPES_MODULE_NAME,
        quote! { #(#declarations)* },
    )]
    .into_iter()
    .chain(shapes.iter().map(render_shape))
    .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use margaret_attribute_arguments::attribute_arguments_error::AttributeArgumentsError;
    use margaret_attributes::attribute_error::AttributeError;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_model_codegen::models::models;
    use margaret_sql_identifier::table_namespace::TableNamespace;

    use super::render_shapes;
    use crate::active_record_codegen_error::ActiveRecordCodegenError;
    use crate::collect_shapes::collect_shapes;

    const MODELS: &str = "use margaret::framework::active_record::children::Children;\nuse margaret::framework::active_record::key::Key;\n\n#[model(table = \"authors\")]\n#[has_many(name = \"articles\", model = Article, key = author)]\n#[has_one(name = \"profile\", model = Profile, key = author)]\nstruct Author {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}\n\n#[model(table = \"profiles\")]\nstruct Profile {\n    #[column(primary_key)]\n    author: Key<Author>,\n}\n\n#[model(table = \"articles\")]\n#[index(name = \"articles_by_author\", fields = [author, id])]\nstruct Article {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    author: Key<Author>,\n    #[column]\n    #[index]\n    editor: Option<Key<Author>>,\n}\n";

    fn outcome(shapes: &str) -> Result<BTreeMap<String, String>, ActiveRecordCodegenError> {
        let indexed = IndexedSource::new(&format!("{MODELS}\n{shapes}"));
        let models =
            models(&indexed.index, TableNamespace::Application).expect("the models resolve");

        collect_shapes(&indexed.index, &models).map(|shapes| {
            render_shapes(&shapes)
                .into_iter()
                .map(|module| {
                    (
                        module.name().to_string(),
                        module.to_source().split_whitespace().collect(),
                    )
                })
                .collect()
        })
    }

    fn rejection(shapes: &str) -> ActiveRecordCodegenError {
        outcome(shapes).expect_err("the shape is rejected")
    }

    fn shape(shapes: &str, name: &str) -> String {
        outcome(shapes)
            .expect("the shapes render")
            .remove(name)
            .expect("the shape module is rendered")
    }

    const WITH_AUTHOR: &str = "#[eager_load(model = Article)]\nstruct ArticleWithAuthor {\n    #[base]\n    article: Article,\n    #[relation(author)]\n    author: Author,\n}\n";

    const WITH_CHILDREN: &str = "#[eager_load(model = Author)]\nstruct AuthorWithFamily {\n    #[base]\n    author: Author,\n    #[relation(articles, limit = 3)]\n    articles: Children<ArticleWithAuthor>,\n    #[relation(profile)]\n    profile: Option<Profile>,\n}\n";

    #[test]
    fn renders_no_modules_without_shapes() {
        assert!(outcome("").expect("nothing renders").is_empty());
    }

    #[test]
    fn declares_one_module_per_shape() {
        assert_eq!(
            outcome(&format!("{WITH_AUTHOR}\n{WITH_CHILDREN}")).expect("the shapes render")["shapes"],
            "pubmodarticle_with_author;pubmodauthor_with_family;"
        );
    }

    #[test]
    fn joins_a_belongs_to_relation() {
        let rendered = shape(WITH_AUTHOR, "shapes/article_with_author");

        assert!(rendered.contains("constJOIN:margaret::framework::active_record::join_context::JoinContext=margaret::framework::active_record::join_context::JoinContext::Required;"));
        assert!(rendered.contains("constWIDTH:usize=<crate::Articleasmargaret::framework::active_record::loadable::Loadable>::WIDTH+<crate::Authorasmargaret::framework::active_record::loadable::Loadable>::WIDTH;"));
        assert!(rendered.contains("builder.belongs_to::<crate::Author,crate::Article>(source,margaret::framework::active_record::field_span::FieldSpan{start:1usize,width:1usize,});"));
        assert!(rendered.contains("article:cursor.record::<crate::Article>()?,"));
        assert!(rendered.contains(
            "implmargaret::framework::active_record::shape::Shapeforcrate::ArticleWithAuthor{}"
        ));
    }

    #[test]
    fn left_joins_an_optional_belongs_to_relation() {
        assert!(
            shape(
                "#[eager_load(model = Article)]\nstruct ArticleWithEditor {\n    #[base]\n    article: Article,\n    #[relation(editor)]\n    editor: Option<Author>,\n}\n",
                "shapes/article_with_editor"
            )
            .contains("builder.belongs_to::<::std::option::Option<crate::Author>,crate::Article>(")
        );
    }

    #[test]
    fn loads_children_after_the_primary_key_of_the_base() {
        let rendered = shape(
            &format!("{WITH_AUTHOR}\n{WITH_CHILDREN}"),
            "shapes/author_with_family",
        );

        assert!(rendered.contains("builder.primary_key::<crate::Author>(source);"));
        assert!(rendered.contains("cursor.skip(<<crate::Authorasmargaret::framework::active_record::record::Record>::PrimaryKeyasmargaret::framework::active_record::value::Value>::WIDTH);"));
        assert!(rendered.contains("articles:margaret::framework::active_record::child_groups::ChildGroups::many(rows,offset+<crate::Authorasmargaret::framework::active_record::loadable::Loadable>::WIDTH,&margaret::framework::active_record::many_relation::ManyRelation{key:margaret::framework::active_record::field_span::FieldSpan{start:1usize,width:1usize,},limit:3usize,order:&[margaret::framework::active_record::field_span::FieldSpan{start:0usize,width:1usize,}],},executor,).await?"));
        assert!(rendered.contains(
            "profile:margaret::framework::active_record::child_groups::ChildGroups::one("
        ));
        assert!(rendered.contains("articles:preloaded.articles.children(cursor.index())"));
        assert!(rendered.contains("profile:preloaded.profile.child(cursor.index())"));
        assert!(rendered.contains("pubarticles:margaret::framework::active_record::child_groups::ChildGroups<crate::ArticleWithAuthor>"));
    }

    #[test]
    fn preloads_the_relations_of_a_joined_shape() {
        let rendered = shape(
            &format!(
                "{WITH_AUTHOR}\n{WITH_CHILDREN}\n#[eager_load(model = Article)]\nstruct ArticleWithFamily {{\n    #[base]\n    article: Article,\n    #[relation(author)]\n    author: AuthorWithFamily,\n}}\n"
            ),
            "shapes/article_with_family",
        );

        assert!(rendered.contains("pubauthor:<crate::AuthorWithFamilyasmargaret::framework::active_record::loadable::Loadable>::Preloaded"));
        assert!(rendered.contains("author:<crate::AuthorWithFamilyasmargaret::framework::active_record::loadable::Loadable>::preload(rows,offset+<crate::Articleasmargaret::framework::active_record::loadable::Loadable>::WIDTH,executor,).await?"));
    }

    #[test]
    fn rejects_a_shape_that_is_not_a_struct() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nenum Loaded {}\n"),
            ActiveRecordCodegenError::EagerLoadNotAStruct { ref shape } if shape == "crate::Loaded"
        ));
    }

    #[test]
    fn rejects_a_model_named_by_a_string() {
        assert!(matches!(
            rejection("#[eager_load(model = \"Article\")]\nstruct Loaded {}\n"),
            ActiveRecordCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "model"
        ));
    }

    #[test]
    fn rejects_a_base_held_by_reference() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: &'static Article,\n}\n"),
            ActiveRecordCodegenError::BaseTypeMismatch { ref field, .. } if field == "article"
        ));
    }

    #[test]
    fn rejects_a_wrapped_relation_held_by_reference() {
        assert!(matches!(
            rejection("#[eager_load(model = Author)]\nstruct Loaded {\n    #[base]\n    author: Author,\n    #[relation(profile)]\n    profile: Option<&'static Profile>,\n}\n"),
            ActiveRecordCodegenError::RelationTypeMismatch { ref field, .. } if field == "profile"
        ));
    }

    #[test]
    fn rejects_malformed_shape_arguments() {
        assert!(matches!(
            rejection("#[eager_load(= 5)]\nstruct Loaded {}\n"),
            ActiveRecordCodegenError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed { ref attribute_path, .. })
            } if attribute_path == "eager_load"
        ));
    }

    #[test]
    fn rejects_a_shape_without_its_model() {
        assert!(matches!(
            rejection("#[eager_load]\nstruct Loaded {}\n"),
            ActiveRecordCodegenError::EagerLoadRequiresModel { ref shape } if shape == "crate::Loaded"
        ));
    }

    #[test]
    fn rejects_a_shape_of_an_unresolvable_model() {
        assert!(matches!(
            rejection("#[eager_load(model = Missing)]\nstruct Loaded {}\n"),
            ActiveRecordCodegenError::EagerLoadModelNotAModel { ref model, .. } if model == "Missing"
        ));
    }

    #[test]
    fn rejects_a_shape_of_a_struct_that_is_not_a_model() {
        assert!(matches!(
            rejection("struct Plain;\n\n#[eager_load(model = Plain)]\nstruct Loaded {}\n"),
            ActiveRecordCodegenError::EagerLoadModelNotAModel { ref model, .. } if model == "Plain"
        ));
    }

    #[test]
    fn rejects_a_positional_shape_field() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded(#[base] Article);\n"),
            ActiveRecordCodegenError::ShapeRequiresNamedFields { position: 0, ref shape } if shape == "crate::Loaded"
        ));
    }

    #[test]
    fn rejects_a_field_marked_both_base_and_relation() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    #[relation(author)]\n    article: Article,\n}\n"),
            ActiveRecordCodegenError::ShapeFieldMarkedTwice { ref field, .. } if field == "article"
        ));
    }

    #[test]
    fn rejects_an_unmarked_shape_field() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    note: String,\n}\n"),
            ActiveRecordCodegenError::ShapeFieldUnmarked { ref field, .. } if field == "note"
        ));
    }

    #[test]
    fn rejects_a_base_of_another_model() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Author,\n}\n"),
            ActiveRecordCodegenError::BaseTypeMismatch { ref found, .. } if found == "Author"
        ));
    }

    #[test]
    fn rejects_an_optional_base() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Option<Article>,\n}\n"),
            ActiveRecordCodegenError::BaseTypeMismatch { ref field, .. } if field == "article"
        ));
    }

    #[test]
    fn rejects_a_shape_without_a_base() {
        assert!(matches!(
            rejection(
                "#[eager_load(model = Article)]\nstruct Loaded {\n    #[relation(author)]\n    author: Author,\n}\n"
            ),
            ActiveRecordCodegenError::ShapeBaseCount { count: 0, ref shape } if shape == "crate::Loaded"
        ));
    }

    #[test]
    fn rejects_a_shape_with_several_bases() {
        assert!(matches!(
            rejection(
                "#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    first: Article,\n    #[base]\n    second: Article,\n}\n"
            ),
            ActiveRecordCodegenError::ShapeBaseCount { count: 2, ref shape } if shape == "crate::Loaded"
        ));
    }

    #[test]
    fn rejects_a_relation_without_a_name() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    #[relation(limit = 2)]\n    author: Author,\n}\n"),
            ActiveRecordCodegenError::RelationRequiresName { ref field, .. } if field == "author"
        ));
    }

    #[test]
    fn rejects_a_relation_named_by_a_path() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    #[relation(self::author)]\n    author: Author,\n}\n"),
            ActiveRecordCodegenError::RelationNameIsNotAnIdentifier { ref relation, .. } if relation == "self::author"
        ));
    }

    #[test]
    fn rejects_a_relation_with_a_malformed_limit() {
        assert!(matches!(
            rejection("#[eager_load(model = Author)]\nstruct Loaded {\n    #[base]\n    author: Author,\n    #[relation(articles, limit = \"two\")]\n    articles: Children<Article>,\n}\n"),
            ActiveRecordCodegenError::AttributeArguments {
                source: AttributeArgumentsError::UnexpectedArgument { ref key, .. }
            } if key == "limit"
        ));
    }

    #[test]
    fn rejects_malformed_relation_arguments() {
        assert!(matches!(
            rejection(
                "#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    #[relation(= 5)]\n    author: Author,\n}\n"
            ),
            ActiveRecordCodegenError::Index {
                source: AttributeError::Arguments(AttributeArgumentsError::Malformed { ref attribute_path, .. })
            } if attribute_path == "relation"
        ));
    }

    #[test]
    fn rejects_an_unknown_relation() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    #[relation(writer)]\n    writer: Author,\n}\n"),
            ActiveRecordCodegenError::UnknownRelation { ref relation, .. } if relation == "writer"
        ));
    }

    #[test]
    fn rejects_a_relation_named_after_a_field_that_is_not_a_key() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    #[relation(id)]\n    id: Author,\n}\n"),
            ActiveRecordCodegenError::UnknownRelation { ref relation, .. } if relation == "id"
        ));
    }

    #[test]
    fn rejects_a_limit_on_a_belongs_to_relation() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    #[relation(author, limit = 2)]\n    author: Author,\n}\n"),
            ActiveRecordCodegenError::LimitOnSingleRelation { ref relation, .. } if relation == "author"
        ));
    }

    #[test]
    fn rejects_a_limit_on_a_has_one_relation() {
        assert!(matches!(
            rejection("#[eager_load(model = Author)]\nstruct Loaded {\n    #[base]\n    author: Author,\n    #[relation(profile, limit = 2)]\n    profile: Option<Profile>,\n}\n"),
            ActiveRecordCodegenError::LimitOnSingleRelation { ref relation, .. } if relation == "profile"
        ));
    }

    #[test]
    fn rejects_a_has_many_relation_without_a_limit() {
        assert!(matches!(
            rejection("#[eager_load(model = Author)]\nstruct Loaded {\n    #[base]\n    author: Author,\n    #[relation(articles)]\n    articles: Children<Article>,\n}\n"),
            ActiveRecordCodegenError::HasManyRelationRequiresLimit { ref relation, .. } if relation == "articles"
        ));
    }

    #[test]
    fn rejects_a_zero_limit() {
        assert!(matches!(
            rejection("#[eager_load(model = Author)]\nstruct Loaded {\n    #[base]\n    author: Author,\n    #[relation(articles, limit = 0)]\n    articles: Children<Article>,\n}\n"),
            ActiveRecordCodegenError::RelationLimitMustBePositive { ref relation, .. } if relation == "articles"
        ));
    }

    #[test]
    fn rejects_a_limit_beyond_the_largest_postgres_limit() {
        assert!(matches!(
            rejection("#[eager_load(model = Author)]\nstruct Loaded {\n    #[base]\n    author: Author,\n    #[relation(articles, limit = 9223372036854775807)]\n    articles: Children<Article>,\n}\n"),
            ActiveRecordCodegenError::RelationLimitExceedsMaximum { limit: 9_223_372_036_854_775_807, ref relation, .. } if relation == "articles"
        ));
    }

    #[test]
    fn rejects_a_relation_held_in_the_wrong_wrapper() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    #[relation(author)]\n    author: Option<Author>,\n}\n"),
            ActiveRecordCodegenError::RelationTypeMismatch { ref expected, .. } if expected == "crate::Author"
        ));
    }

    #[test]
    fn rejects_a_relation_holding_another_model() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    #[relation(author)]\n    author: Profile,\n}\n"),
            ActiveRecordCodegenError::RelationTypeMismatch { ref found, .. } if found == "Profile"
        ));
    }

    #[test]
    fn rejects_a_relation_held_in_an_unresolvable_type() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    #[relation(author)]\n    author: [u8; 4],\n}\n"),
            ActiveRecordCodegenError::RelationTypeMismatch { ref field, .. } if field == "author"
        ));
    }

    #[test]
    fn rejects_a_relation_held_in_an_undeclared_type() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    #[relation(author)]\n    author: Missing,\n}\n"),
            ActiveRecordCodegenError::RelationTypeMismatch { ref found, .. } if found == "Missing"
        ));
    }

    #[test]
    fn rejects_a_wrapped_relation_of_an_undeclared_type() {
        assert!(matches!(
            rejection("#[eager_load(model = Author)]\nstruct Loaded {\n    #[base]\n    author: Author,\n    #[relation(profile)]\n    profile: Option<Missing>,\n}\n"),
            ActiveRecordCodegenError::RelationTypeMismatch { ref found, .. } if found == "Option < Missing >"
        ));
    }

    #[test]
    fn rejects_a_wrapper_without_its_argument() {
        assert!(matches!(
            rejection("#[eager_load(model = Author)]\nstruct Loaded {\n    #[base]\n    author: Author,\n    #[relation(profile)]\n    profile: Option,\n}\n"),
            ActiveRecordCodegenError::RelationTypeMismatch { ref field, .. } if field == "profile"
        ));
    }

    #[test]
    fn rejects_a_relation_declared_twice() {
        assert!(matches!(
            rejection("#[eager_load(model = Article)]\nstruct Loaded {\n    #[base]\n    article: Article,\n    #[relation(author)]\n    first: Author,\n    #[relation(author)]\n    second: Author,\n}\n"),
            ActiveRecordCodegenError::DuplicateShapeRelation { ref relation, .. } if relation == "author"
        ));
    }
}
