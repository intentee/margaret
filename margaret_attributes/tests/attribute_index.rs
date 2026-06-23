use std::path::Path;
use std::path::PathBuf;

use margaret_attributes::AttributeIndex;
use margaret_attributes::AttributeSelector;
use margaret_attributes::ItemKind;
use margaret_attributes::SelectedItem;

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn valid_index() -> AttributeIndex {
    AttributeIndex::from_crate_root("valid_crate", &fixture("valid_crate"))
        .expect("the valid fixture indexes cleanly")
}

fn selector(input: &str) -> AttributeSelector {
    AttributeSelector::parse(input).expect("the selector parses")
}

fn selected_paths(selected: &[SelectedItem]) -> Vec<String> {
    selected
        .iter()
        .map(|item| item.canonical_path().to_string())
        .collect()
}

fn index_error(fixture_name: &str) -> String {
    AttributeIndex::from_crate_root(fixture_name, &fixture(fixture_name))
        .err()
        .expect("the fixture fails to index")
        .to_string()
}

#[test]
fn indexes_every_item_across_inline_and_file_modules() {
    let index = valid_index();

    let paths: Vec<String> = index
        .items()
        .iter()
        .map(|item| item.canonical_path().to_string())
        .collect();

    assert!(paths.contains(&"valid_crate::RootStruct".to_string()));
    assert!(paths.contains(&"valid_crate::inline_module::InsideInline".to_string()));
    assert!(paths.contains(&"valid_crate::file_module::InFileModule".to_string()));
    assert!(paths.contains(&"valid_crate::file_module::nested::DeeplyNested".to_string()));
    assert!(paths.contains(&"valid_crate::dir_module::InDirModule".to_string()));
}

#[test]
fn items_are_sorted_by_canonical_path() {
    let index = valid_index();

    let paths: Vec<String> = index
        .items()
        .iter()
        .map(|item| item.canonical_path().to_string())
        .collect();

    let mut sorted = paths.clone();
    sorted.sort();

    assert_eq!(paths, sorted);
}

#[test]
fn records_each_item_kind() {
    let index = valid_index();

    let kinds: Vec<ItemKind> = index.items().iter().map(|item| item.kind()).collect();

    assert!(kinds.contains(&ItemKind::Struct));
    assert!(kinds.contains(&ItemKind::Enum));
    assert!(kinds.contains(&ItemKind::Function));
    assert!(kinds.contains(&ItemKind::Trait));
    assert!(kinds.contains(&ItemKind::Module));
}

#[test]
fn ignores_items_that_are_not_named_definitions() {
    let index = valid_index();

    let paths: Vec<String> = index
        .items()
        .iter()
        .map(|item| item.canonical_path().to_string())
        .collect();

    assert!(!paths.contains(&"valid_crate::ROOT_CONST".to_string()));
    assert!(!paths.contains(&"valid_crate::RootAlias".to_string()));
}

#[test]
fn exposes_item_identifier() {
    let index = valid_index();

    let identifier = index
        .items()
        .iter()
        .find(|item| item.canonical_path().to_string() == "valid_crate::RootStruct")
        .expect("RootStruct is indexed")
        .identifier();

    assert_eq!(identifier, "RootStruct");
}

#[test]
fn selects_items_bearing_an_attribute() {
    let index = valid_index();

    let paths = selected_paths(&index.select(&selector("singleton")));

    assert!(paths.contains(&"valid_crate::RootStruct".to_string()));
    assert!(paths.contains(&"valid_crate::WithProvides".to_string()));
    assert!(!paths.contains(&"valid_crate::RootTrait".to_string()));
}

#[test]
fn selects_one_entry_per_repeated_attribute_occurrence() {
    let index = valid_index();

    let occurrences = selected_paths(&index.select(&selector("singleton")))
        .into_iter()
        .filter(|path| path == "valid_crate::RepeatedAttrs")
        .count();

    assert_eq!(occurrences, 2);
}

#[test]
fn selected_item_exposes_kind_and_canonical_path() {
    let index = valid_index();
    let selected = index.select(&selector("ns::tagged"));
    let item = selected.first().expect("the qualified item is selected");

    assert_eq!(item.kind(), ItemKind::Struct);
    assert_eq!(item.canonical_path().to_string(), "valid_crate::Qualified");
    assert_eq!(
        item.canonical_path().segments(),
        ["valid_crate".to_string(), "Qualified".to_string()]
    );
}

#[test]
fn matches_attribute_by_suffix_regardless_of_qualification() {
    let index = valid_index();

    let by_leaf = selected_paths(&index.select(&selector("tagged")));
    let by_full = selected_paths(&index.select(&selector("ns::tagged")));

    assert!(by_leaf.contains(&"valid_crate::Qualified".to_string()));
    assert!(by_full.contains(&"valid_crate::Qualified".to_string()));
}

#[test]
fn selector_longer_than_attribute_path_matches_nothing() {
    let index = valid_index();

    assert!(
        index
            .select(&selector("deeply::nested::singleton"))
            .is_empty()
    );
}

#[test]
fn selector_with_unknown_name_matches_nothing() {
    let index = valid_index();

    assert!(index.select(&selector("does_not_exist")).is_empty());
}

#[test]
fn formats_matched_attribute_path() {
    let index = valid_index();
    let qualified = index.select(&selector("ns::tagged"));
    let plain = index.select(&selector("singleton"));

    assert_eq!(
        qualified
            .first()
            .expect("qualified item selected")
            .attribute()
            .path(),
        "ns::tagged"
    );
    assert_eq!(
        plain
            .first()
            .expect("plain item selected")
            .attribute()
            .path(),
        "singleton"
    );
}

#[test]
fn reads_attribute_without_arguments_as_empty() {
    let index = valid_index();
    let selected = index.select(&selector("singleton"));
    let root = selected
        .iter()
        .find(|item| item.canonical_path().to_string() == "valid_crate::RootStruct")
        .expect("RootStruct selected");

    let arguments = root.attribute().args().expect("empty arguments parse");

    assert!(arguments.is_empty());
}

#[test]
fn reads_named_and_positional_arguments() {
    let index = valid_index();
    let selected = index.select(&selector("responds_to_http"));
    let function = selected.first().expect("function selected");
    let arguments = function.attribute().args().expect("arguments parse");

    assert!(!arguments.is_empty());
    assert!(arguments.named("method").is_some());
    assert!(arguments.named("absent").is_none());
    assert!(arguments.positional(0).is_some());
    assert!(arguments.positional(99).is_none());
}

#[test]
fn reads_string_argument_and_rejects_other_kinds() {
    let index = valid_index();
    let selected = index.select(&selector("responds_to_http"));
    let arguments = selected
        .first()
        .expect("function selected")
        .attribute()
        .args()
        .expect("arguments parse");

    assert_eq!(
        arguments.string("pattern").expect("string argument"),
        Some("/home".to_string())
    );
    assert_eq!(arguments.string("absent").expect("absent argument"), None);
    assert!(arguments.string("count").is_err());
    assert!(arguments.string("method").is_err());
}

#[test]
fn reads_path_argument_and_rejects_other_kinds() {
    let index = valid_index();
    let selected = index.select(&selector("responds_to_http"));
    let arguments = selected
        .first()
        .expect("function selected")
        .attribute()
        .args()
        .expect("arguments parse");

    assert!(arguments.path("method").expect("path argument").is_some());
    assert!(arguments.path("absent").expect("absent argument").is_none());
    assert!(arguments.path("pattern").is_err());
}

#[test]
fn reads_name_value_attribute_argument() {
    let index = valid_index();
    let selected = index.select(&selector("doc"));
    let arguments = selected
        .first()
        .expect("documented item selected")
        .attribute()
        .args()
        .expect("name value arguments parse");

    assert!(arguments.named("doc").is_some());
}

#[test]
fn reports_malformed_attribute_arguments() {
    let index = valid_index();
    let selected = index.select(&selector("bad_args"));
    let message = selected
        .first()
        .expect("item with malformed arguments selected")
        .attribute()
        .args()
        .err()
        .expect("malformed arguments fail to parse")
        .to_string();

    assert!(message.contains("could not be parsed"));
}

#[test]
fn finds_single_sibling_attribute() {
    let index = valid_index();
    let selected = index.select(&selector("singleton"));
    let root = selected
        .iter()
        .find(|item| item.canonical_path().to_string() == "valid_crate::RootStruct")
        .expect("RootStruct selected");
    let found = root
        .attributes()
        .find(&selector("singleton"))
        .expect("sibling lookup succeeds");

    assert!(found.is_some());
}

#[test]
fn missing_sibling_attribute_returns_none() {
    let index = valid_index();
    let selected = index.select(&selector("singleton"));
    let found = selected
        .first()
        .expect("an item is selected")
        .attributes()
        .find(&selector("does_not_exist"))
        .expect("sibling lookup succeeds");

    assert!(found.is_none());
}

#[test]
fn repeated_sibling_attribute_is_rejected() {
    let index = valid_index();
    let selected = index.select(&selector("singleton"));
    let repeated = selected
        .iter()
        .find(|item| item.canonical_path().to_string() == "valid_crate::RepeatedAttrs")
        .expect("RepeatedAttrs selected");

    let message = repeated
        .attributes()
        .find(&selector("singleton"))
        .err()
        .expect("repeated sibling is rejected")
        .to_string();

    assert!(message.contains("is repeated"));
}

#[test]
fn finds_all_matching_sibling_attributes() {
    let index = valid_index();
    let selected = index.select(&selector("singleton"));
    let repeated = selected
        .iter()
        .find(|item| item.canonical_path().to_string() == "valid_crate::RepeatedAttrs")
        .expect("RepeatedAttrs selected");

    assert_eq!(
        repeated.attributes().find_all(&selector("singleton")).len(),
        2
    );
    assert!(
        repeated
            .attributes()
            .find_all(&selector("does_not_exist"))
            .is_empty()
    );
}

#[test]
fn rejects_invalid_selector() {
    let message = AttributeSelector::parse("123")
        .err()
        .expect("an invalid selector is rejected")
        .to_string();

    assert!(message.contains("invalid attribute selector"));
}

#[test]
fn reports_missing_crate_root() {
    let message = AttributeIndex::from_crate_root("ghost_crate", &fixture("does_not_exist"))
        .err()
        .expect("a missing crate root fails")
        .to_string();

    assert!(message.contains("failed to read"));
}

#[test]
fn reports_invalid_root_syntax() {
    assert!(index_error("bad_syntax").contains("failed to parse"));
}

#[test]
fn reports_invalid_submodule_syntax() {
    assert!(index_error("nested_bad_syntax").contains("failed to parse"));
}

#[test]
fn rejects_duplicate_canonical_paths() {
    assert!(index_error("duplicate").contains("same canonical path"));
}

#[test]
fn rejects_glob_imports() {
    assert!(index_error("glob").contains("glob import"));
}

#[test]
fn rejects_glob_imports_inside_inline_modules() {
    assert!(index_error("inline_glob").contains("glob import"));
}

#[test]
fn rejects_path_attribute_modules() {
    assert!(index_error("path_attr").contains("#[path]"));
}

#[test]
fn rejects_ambiguous_module_files() {
    assert!(index_error("module_collision").contains("resolves to both"));
}

#[test]
fn reports_missing_module_files() {
    assert!(index_error("module_missing").contains("no source file"));
}
