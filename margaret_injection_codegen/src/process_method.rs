use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;

use crate::injection_error::InjectionError;

/// # Errors
///
/// Returns `InjectionError::AmbiguousProcessMethod` or `InjectionError::MissingProcessMethod`.
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
    use margaret_attributes_tests::indexed_source::IndexedSource;

    use super::process_method;

    #[test]
    fn returns_the_single_process_method_so_the_responder_has_one_entry_point() {
        let indexed = IndexedSource::new(
            r"use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

#[singleton]
struct One;

impl One {
    #[process]
    fn run(&self) -> Response {}
}
",
        );

        let method = process_method(indexed.item("One")).expect("the process method is found");

        assert_eq!(method.identifier(), "run");
    }

    #[test]
    fn reports_a_missing_process_method_since_a_responder_needs_an_entry_point() {
        let indexed = IndexedSource::new(
            r"use margaret::framework::macros::constructor;
use margaret::framework::macros::singleton;

#[singleton]
struct Bare;

impl Bare {
    #[constructor]
    fn create() -> Self {}
}
",
        );

        let error = process_method(indexed.item("Bare"))
            .err()
            .expect("the missing process method is reported");

        assert!(error.to_string().contains("no #[process] method"));
    }

    #[test]
    fn reports_an_ambiguous_process_method_instead_of_picking_one_of_them() {
        let indexed = IndexedSource::new(
            r"use margaret::framework::macros::process;
use margaret::framework::macros::singleton;

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

        let error = process_method(indexed.item("Two"))
            .err()
            .expect("the ambiguous process method is reported");

        assert!(
            error
                .to_string()
                .contains("more than one #[process] method")
        );
    }
}
