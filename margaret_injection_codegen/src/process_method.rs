use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;

use crate::injection_error::InjectionError;

pub fn process_method(item: &IndexedItem) -> Result<&IndexedMethod, InjectionError> {
    let mut found: Vec<&IndexedMethod> = item
        .methods()
        .iter()
        .filter(|method| method.has_framework_attribute(FrameworkAttribute::Process))
        .collect();

    if found.len() > 1 {
        return Err(InjectionError::AmbiguousProcessMethod {
            item: item.canonical_path().to_string(),
            methods: found
                .iter()
                .map(|method| method.identifier().to_string())
                .collect::<Vec<String>>()
                .join(", "),
        });
    }

    found
        .pop()
        .ok_or_else(|| InjectionError::MissingProcessMethod {
            item: item.canonical_path().to_string(),
        })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_attributes::indexed_item::IndexedItem;

    use super::process_method;

    fn index_for(lib_source: &str) -> AttributeIndex {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", source_directory))
            .expect("the crate is indexed")
            .build()
    }

    fn item_named<'index>(index: &'index AttributeIndex, identifier: &str) -> &'index IndexedItem {
        index
            .items()
            .iter()
            .find(|item| item.identifier() == identifier)
            .expect("the item is indexed")
    }

    #[test]
    fn returns_the_single_process_method() {
        let index = index_for(
            r"
#[singleton]
struct One;

impl One {
    #[process]
    fn run(&self) -> Response {}
}
",
        );

        let method =
            process_method(item_named(&index, "One")).expect("the process method is found");

        assert_eq!(method.identifier(), "run");
    }

    #[test]
    fn reports_a_missing_process_method() {
        let index = index_for(
            r"
#[singleton]
struct Bare;

impl Bare {
    #[constructor]
    fn create() -> Self {}
}
",
        );

        let error = process_method(item_named(&index, "Bare"))
            .err()
            .expect("the missing process method is reported");

        assert!(error.to_string().contains("no #[process] method"));
    }

    #[test]
    fn reports_an_ambiguous_process_method() {
        let index = index_for(
            r"
#[singleton]
struct Two;

impl Two {
    #[process]
    fn first(&self) -> Response {}

    #[process]
    fn second(&self) -> Response {}
}
",
        );

        let error = process_method(item_named(&index, "Two"))
            .err()
            .expect("the ambiguous process method is reported");

        assert!(
            error
                .to_string()
                .contains("more than one #[process] method")
        );
    }
}
