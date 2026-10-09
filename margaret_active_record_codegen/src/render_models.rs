use std::collections::HashSet;

use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_model_codegen::field_value::FieldValue;
use margaret_model_codegen::model::Model;
use margaret_schema_codegen::models_module_name::MODELS_MODULE_NAME;

use crate::render_model_module::render_model_module;

#[must_use]
pub fn render_models(models: &[Model]) -> Vec<GeneratedModuleTokens> {
    let key_targets: HashSet<CanonicalPath> = models
        .iter()
        .flat_map(|model| &model.fields)
        .filter_map(|field| match &field.value {
            FieldValue::Key { target, .. } => Some(target.clone()),
            FieldValue::Enum { .. } | FieldValue::Json { .. } | FieldValue::Scalar { .. } => None,
        })
        .collect();
    let mut rendered_enums: HashSet<CanonicalPath> = HashSet::new();
    let declarations = models.iter().map(|model| {
        let module = format_ident!("{}", model.module);

        quote! { pub mod #module; }
    });
    let mut modules = vec![GeneratedModuleTokens::new(
        MODELS_MODULE_NAME,
        quote! { #(#declarations)* },
    )];

    for model in models {
        modules.extend(render_model_module(
            model,
            &key_targets,
            &mut rendered_enums,
        ));
    }

    modules
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_model_codegen::models::models;
    use margaret_sql_identifier::table_namespace::TableNamespace;

    use super::render_models;

    const KEY: &str = "use margaret::framework::active_record::key::Key;\n";

    fn rendered(lib_source: &str) -> BTreeMap<String, String> {
        let indexed = IndexedSource::new(lib_source);

        render_models(
            &models(&indexed.index, TableNamespace::Application).expect("the models resolve"),
        )
        .into_iter()
        .map(|module| {
            (
                module.name().to_string(),
                module.to_source().split_whitespace().collect(),
            )
        })
        .collect()
    }

    fn module(lib_source: &str, name: &str) -> String {
        rendered(lib_source)
            .remove(name)
            .expect("the module is rendered")
    }

    const LOCK: &str = "#[model(table = \"locks\")]\n#[primary_key(fields = [repository, branch])]\nstruct Lock {\n    #[column]\n    repository: String,\n    #[column]\n    branch: String,\n}\n";

    const RELEASE: &str = "use margaret::framework::active_record::key::Key;\n\n#[model(table = \"releases\")]\nstruct Release {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    #[index]\n    lock: Key<Lock>,\n}\n";

    const COUNTER: &str = "#[model(table = \"counters\")]\nstruct Counter {\n    #[column(primary_key)]\n    name: String,\n    #[column]\n    hits: i64,\n}\n";

    const ARTICLE: &str = "use margaret::framework::active_record::key::Key;\n\nenum Status {\n    Draft,\n    Published,\n}\n\n#[model(table = \"authors\")]\nstruct Author {\n    #[column(primary_key, default = margaret::framework::model::column_default::ColumnDefault::UuidV7)]\n    id: uuid::Uuid,\n    #[column]\n    status: Status,\n}\n\n#[model(table = \"articles\")]\n#[index(name = \"articles_by_author\", fields = [author, id])]\n#[index(name = \"articles_by_author_title\", fields = [author, title])]\nstruct Article {\n    #[column(primary_key, default = margaret::framework::model::column_default::ColumnDefault::UuidV7)]\n    id: uuid::Uuid,\n    #[column]\n    title: String,\n    #[column]\n    status: Status,\n    #[column]\n    author: Key<Author>,\n}\n";

    #[test]
    fn declares_one_module_per_model() {
        assert_eq!(module(ARTICLE, "models"), "pubmodauthor;pubmodarticle;");
    }

    #[test]
    fn declares_the_modules_of_a_creatable_model() {
        assert_eq!(
            module(ARTICLE, "models/author"),
            "pubmodcolumns;pubmodconditions;pubmoddraft;modenum_columns;modmodel;pubmodquery;modrecord;pubmodtable;"
        );
    }

    #[test]
    fn declares_the_modules_of_a_model_without_database_defaults() {
        assert_eq!(
            module(COUNTER, "models/counter"),
            "pubmodcolumns;pubmodconditions;modmodel;pubmodquery;modrecord;pubmodtable;"
        );
    }

    #[test]
    fn declares_a_composite_primary_key_module_without_columns() {
        assert_eq!(
            module(LOCK, "models/lock"),
            "pubmodconditions;modmodel;pubmodprimary_key;pubmodquery;modrecord;pubmodtable;"
        );
    }

    #[test]
    fn renders_the_codec_of_an_enum_once() {
        let modules = rendered(ARTICLE);

        assert!(modules["models/author/enum_columns"].contains(
            "implmargaret::framework::active_record::enum_column::EnumColumnforcrate::Status{constENUM_TYPE:&'staticstr=\"crate::Status\";fnfrom_variant_name(stored:&str)->::std::option::Option<Self>{matchstored{\"Draft\"=>::std::option::Option::Some(Self::Draft),\"Published\"=>::std::option::Option::Some(Self::Published),_=>::std::option::Option::None,}}fnvariant_name(&self)->&'staticstr{matchself{Self::Draft=>\"Draft\",Self::Published=>\"Published\",}}}"
        ));
        assert!(!modules.contains_key("models/article/enum_columns"));
    }

    #[test]
    fn reads_and_writes_the_fields_of_a_record() {
        let record = module(COUNTER, "models/counter/record");

        assert!(record.contains("typePrimaryKey=std::string::String;"));
        assert!(record.contains(
            "constPRIMARY_KEY:&'static[margaret::framework::active_record::field_span::FieldSpan]=&[margaret::framework::active_record::field_span::FieldSpan{start:0usize,width:1usize,}];"
        ));
        assert!(record.contains("constTABLE:&'staticmargaret::framework::model::table::Table=&crate::margaret::models::counter::table::TABLE;"));
        assert!(!record.contains("KeyTarget"));
        assert!(record.contains("margaret::framework::active_record::field::Field::read(cursor).and_then(|value_0|margaret::framework::active_record::field::Field::read(cursor).map(|value_1|Self{name:value_0,hits:value_1}))"));
        assert!(record.contains(
            "::std::result::Result::Ok(()).and_then(|()|margaret::framework::active_record::field::Field::write(&self.name,parameters)).and_then(|()|margaret::framework::active_record::field::Field::write(&self.hits,parameters))"
        ));
    }

    #[test]
    fn assembles_a_composite_primary_key() {
        let source = format!("{LOCK}{RELEASE}");

        assert!(module(&source, "models/lock/record").contains(
            "implmargaret::framework::active_record::key_target::KeyTargetforcrate::Lock{fnprimary_key(&self)->Self::PrimaryKey{crate::margaret::models::lock::primary_key::PrimaryKey{repository:::std::clone::Clone::clone(&self.repository),branch:::std::clone::Clone::clone(&self.branch)}}}"
        ));
        assert!(module(&source, "models/lock/primary_key").contains(
            "constWIDTH:usize=<std::string::Stringasmargaret::framework::active_record::value::Value>::WIDTH+<std::string::Stringasmargaret::framework::active_record::value::Value>::WIDTH;"
        ));
        assert!(module(&source, "models/lock/primary_key").contains(
            "#[derive(Clone,Debug,PartialEq)]pubstructPrimaryKey{pubrepository:std::string::String,pubbranch:std::string::String}"
        ));
    }

    #[test]
    fn declares_an_unreferenced_composite_primary_key_without_encoding_it() {
        let primary_key = module(LOCK, "models/lock/primary_key");

        assert!(primary_key.contains(
            "#[derive(Clone,Debug,PartialEq)]pubstructPrimaryKey{pubrepository:std::string::String,pubbranch:std::string::String}"
        ));
        assert!(!primary_key.contains("Value"));
        assert!(!module(LOCK, "models/lock/record").contains("KeyTarget"));
    }

    #[test]
    fn references_the_primary_key_of_a_key_target() {
        assert!(module(ARTICLE, "models/author/record").contains(
            "implmargaret::framework::active_record::key_target::KeyTargetforcrate::Author{fnprimary_key(&self)->Self::PrimaryKey{::std::clone::Clone::clone(&self.id)}}"
        ));
    }

    #[test]
    fn excludes_defaulted_fields_from_the_draft() {
        assert!(
            module(ARTICLE, "models/article/draft").contains(
                "pubstructDraft{pubtitle:std::string::String,pubstatus:crate::Status,pubauthor:margaret::framework::active_record::key::Key<crate::Author>}"
            )
        );
    }

    #[test]
    fn assigns_only_fields_outside_the_primary_key() {
        assert!(module(COUNTER, "models/counter/columns").contains(
            "pubstructColumns<Context>{pubhits:margaret::framework::active_record::column::Column<crate::Counter,i64,Context>}"
        ));
    }

    #[test]
    fn implements_the_model_traits() {
        let model = module(ARTICLE, "models/article/model");

        assert!(model.contains("typeConditions=crate::margaret::models::article::conditions::Conditions;typeQuery=crate::margaret::models::article::query::Step;"));
        assert!(model.contains(
            "implmargaret::framework::active_record::assignable::Assignableforcrate::Article"
        ));
        assert!(model.contains("implmargaret::framework::active_record::creatable::Creatableforcrate::Article{typeDraft=crate::margaret::models::article::draft::Draft;}"));
    }

    #[test]
    fn omits_assignment_from_a_model_made_of_its_primary_key() {
        assert!(!module(LOCK, "models/lock/model").contains("Assignable"));
    }

    #[test]
    fn walks_the_leading_fields_of_every_index_from_the_query_root() {
        let query = module(ARTICLE, "models/article/query");

        assert!(query.contains("pubmodedge;pubmodthen;"));
        assert!(query.contains("pubid:margaret::framework::active_record::next::Next<crate::Article,uuid::Uuid,crate::margaret::models::article::query::edge::id::Edge,margaret::framework::active_record::root::Root>"));
        assert!(query.contains("pubauthor:margaret::framework::active_record::next::Next<crate::Article,margaret::framework::active_record::key::Key<crate::Author>,crate::margaret::models::article::query::edge::author::Edge,margaret::framework::active_record::root::Root>"));
        assert!(query.contains("id:margaret::framework::active_record::next::Next::new(margaret::framework::active_record::root::Root,0usize)"));
    }

    #[test]
    fn narrows_a_branch_into_each_of_its_indexes() {
        let branch = module(ARTICLE, "models/article/query/then/author");

        assert!(module(ARTICLE, "models/article/query/then/author/edge/id").contains("typeContinued=margaret::framework::active_record::unique::Unique<crate::Article,margaret::framework::active_record::unguarded::Unguarded,>;"));
        assert!(
            module(ARTICLE, "models/article/query/then/author/edge/title").contains(
                "typeContinued=margaret::framework::active_record::prefix::Prefix<crate::Article>;"
            )
        );
        assert!(branch.contains("title:margaret::framework::active_record::next::Next::new(::std::clone::Clone::clone(&narrowed),1usize,)"));
        assert!(
            branch.contains(
                "id:margaret::framework::active_record::next::Next::new(narrowed,0usize)"
            )
        );
        assert!(branch.contains("pubasyncfndelete"));
        assert!(branch.contains("pubasyncfnupdate"));
    }

    #[test]
    fn leaves_a_branch_with_several_continuations_unordered() {
        assert!(!module(ARTICLE, "models/article/query/edge/author").contains("ScanOrder"));
    }

    #[test]
    fn orders_a_unique_field_by_itself() {
        assert!(module(ARTICLE, "models/article/query/edge/id").contains(
            "implmargaret::framework::active_record::scan_order::ScanOrder<crate::Article>forEdge{constSPANS:&'static[margaret::framework::active_record::field_span::FieldSpan]=&[margaret::framework::active_record::field_span::FieldSpan{start:0usize,width:1usize,}];}"
        ));
    }

    #[test]
    fn orders_a_field_by_itself_and_the_rest_of_its_unique_path() {
        let source = format!(
            "{KEY}#[model(table = \"authors\")]\nstruct Author {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n}}\n\n#[model(table = \"articles\")]\n#[index(name = \"articles_by_author\", fields = [author, id])]\nstruct Article {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    author: Key<Author>,\n}}\n"
        );

        assert!(module(&source, "models/article/query/edge/author").contains(
            "constSPANS:&'static[margaret::framework::active_record::field_span::FieldSpan]=&[margaret::framework::active_record::field_span::FieldSpan{start:1usize,width:1usize,},margaret::framework::active_record::field_span::FieldSpan{start:0usize,width:1usize,}];"
        ));
    }

    #[test]
    fn names_the_module_of_a_raw_identifier_field_after_its_identifier() {
        let source = "#[model(table = \"tokens\")]\n#[index(name = \"tokens_by_type\", fields = [r#type, id])]\nstruct Token {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column(name = \"kind\")]\n    r#type: i64,\n}\n";
        let modules = rendered(source);

        assert!(modules["models/token/query/edge"].contains("pubmodr#type;"));
        assert!(modules.contains_key("models/token/query/edge/type"));
        assert!(modules.contains_key("models/token/query/then/type"));
        assert!(modules["models/token/query"].contains(
            "pubr#type:margaret::framework::active_record::next::Next<crate::Token,i64,crate::margaret::models::token::query::edge::r#type::Edge,"
        ));
    }

    #[test]
    fn leaves_a_branch_through_a_nullable_field_unordered() {
        let source = format!(
            "{KEY}#[model(table = \"nodes\")]\n#[index(name = \"nodes_by_label\", fields = [label, parent, id])]\nstruct Node {{\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    label: String,\n    #[column]\n    #[index]\n    parent: Option<Key<Node>>,\n}}\n"
        );

        assert!(!module(&source, "models/node/query/edge/label").contains("ScanOrder"));
    }

    #[test]
    fn leaves_a_branch_ending_in_a_plain_index_unordered() {
        let source = "#[model(table = \"events\")]\n#[index(name = \"events_by_kind\", fields = [kind, label])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n    #[column]\n    label: String,\n}\n";

        assert!(!module(source, "models/event/query/edge/kind").contains("ScanOrder"));
    }

    #[test]
    fn omits_bulk_updates_from_the_branches_of_a_model_made_of_its_primary_key() {
        let source = LOCK;

        assert!(!module(source, "models/lock/query/then/repository").contains("update"));
    }

    #[test]
    fn nests_the_branches_of_a_deep_index() {
        let source = "#[model(table = \"events\")]\n#[index(name = \"events_by_kind\", fields = [kind, label, id])]\nstruct Event {\n    #[column(primary_key)]\n    id: uuid::Uuid,\n    #[column]\n    kind: String,\n    #[column]\n    label: String,\n}\n";
        let modules = rendered(source);

        assert_eq!(modules["models/event/query/then"], "pubmodkind;");
        assert_eq!(modules["models/event/query/then/kind/then"], "pubmodlabel;");
        assert!(modules.contains_key("models/event/query/then/kind/then/label"));
    }

    #[test]
    fn places_each_column_at_the_offset_of_its_field() {
        assert!(module(COUNTER, "models/counter/columns").contains(
            "constFIELDS:Self=Self{hits:margaret::framework::active_record::column::Column{field:::std::marker::PhantomData,start:1usize,}};"
        ));
    }

    #[test]
    fn places_each_operand_at_the_offset_of_its_field() {
        assert!(module(COUNTER, "models/counter/conditions").contains(
            "constFIELDS:Self=Self{name:margaret::framework::active_record::operand::Operand{field:::std::marker::PhantomData,start:0usize,},hits:margaret::framework::active_record::operand::Operand{field:::std::marker::PhantomData,start:1usize,}};"
        ));
    }

    #[test]
    fn wraps_optional_json_and_scalar_fields_in_their_types() {
        let source = "#[model(table = \"payloads\")]\nstruct Payload {\n    #[column(primary_key)]\n    id: i64,\n    #[column]\n    document: Option<margaret::framework::active_record::json::Json<std::collections::BTreeMap<String, i64>>>,\n}\n";

        assert!(module(source, "models/payload/conditions").contains(
            "pubdocument:margaret::framework::active_record::operand::Operand<crate::Payload,::std::option::Option<margaret::framework::active_record::json::Json<std::collections::BTreeMap<std::string::String,i64>>>>"
        ));
    }
}
