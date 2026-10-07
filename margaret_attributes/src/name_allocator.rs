use std::collections::HashSet;

use heck::ToUpperCamelCase;

use crate::identifier::Identifier;
use crate::is_identifier::is_identifier;

#[derive(Default)]
pub struct NameAllocator {
    taken: HashSet<String>,
}

impl NameAllocator {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allocate(&mut self, base: &str) -> Identifier {
        let mut candidate = base.to_string();
        let mut ordinal = 1;

        loop {
            let type_name = candidate.to_upper_camel_case();

            if is_identifier(&candidate)
                && is_identifier(&type_name)
                && self.taken.insert(type_name.clone())
            {
                return Identifier::new(candidate, type_name);
            }

            ordinal += 1;
            candidate = format!("{base}_{ordinal}");
        }
    }

    pub fn reserve(&mut self, base: &str) {
        self.taken.insert(base.to_upper_camel_case());
    }
}

#[cfg(test)]
mod tests {
    use super::NameAllocator;

    #[test]
    fn keeps_a_free_base_unchanged() {
        let mut allocator = NameAllocator::new();
        let identifier = allocator.allocate("routes_get_greeting");

        assert_eq!(identifier.field(), "routes_get_greeting");
        assert_eq!(identifier.type_name(), "RoutesGetGreeting");
    }

    #[test]
    fn disambiguates_a_repeated_base_with_an_ordinal() {
        let mut allocator = NameAllocator::new();

        assert_eq!(allocator.allocate("handler").field(), "handler");
        assert_eq!(allocator.allocate("handler").field(), "handler_2");
        assert_eq!(allocator.allocate("handler").field(), "handler_3");
    }

    #[test]
    fn disambiguates_bases_that_share_an_upper_camel_case_form() {
        let mut allocator = NameAllocator::new();
        let first = allocator.allocate("v2");
        let second = allocator.allocate("v_2");

        assert_eq!(first.field(), "v2");
        assert_eq!(first.type_name(), "V2");
        assert_eq!(second.field(), "v_2_2");
        assert_eq!(second.type_name(), "V22");
    }

    #[test]
    fn steps_over_a_keyword() {
        let mut allocator = NameAllocator::new();
        let identifier = allocator.allocate("self");

        assert_eq!(identifier.field(), "self_2");
        assert_eq!(identifier.type_name(), "Self2");
    }

    #[test]
    fn steps_over_a_reserved_type_name() {
        let mut allocator = NameAllocator::new();

        allocator.reserve("Widget");

        assert_eq!(allocator.allocate("widget").type_name(), "Widget2");
    }
}
