use std::collections::HashMap;
use std::collections::hash_map::Entry;

use crate::named_value::NamedValue;

pub(crate) enum UniqueIndex<Value> {
    Duplicated { name: String },
    Indexed(HashMap<String, Value>),
}

impl<Value> UniqueIndex<Value> {
    pub(crate) fn of(entries: Vec<NamedValue<Value>>) -> Self {
        let mut indexed = HashMap::with_capacity(entries.len());

        for NamedValue { name, value } in entries {
            match indexed.entry(name) {
                Entry::Occupied(occupied) => {
                    return Self::Duplicated {
                        name: occupied.key().clone(),
                    };
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(value);
                }
            }
        }

        Self::Indexed(indexed)
    }
}

#[cfg(test)]
mod tests {
    use super::UniqueIndex;
    use crate::named_value::NamedValue;

    fn named(name: &str, value: u8) -> NamedValue<u8> {
        NamedValue {
            name: name.to_string(),
            value,
        }
    }

    #[test]
    fn indexes_distinct_names() {
        assert!(matches!(
            UniqueIndex::of(vec![named("a", 1), named("b", 2)]),
            UniqueIndex::Indexed(indexed) if indexed.get("b") == Some(&2)
        ));
    }

    #[test]
    fn reports_the_first_repeated_name() {
        assert!(matches!(
            UniqueIndex::of(vec![named("a", 1), named("b", 2), named("a", 3)]),
            UniqueIndex::Duplicated { name } if name == "a"
        ));
    }
}
