use std::collections::BTreeMap;

use proc_macro2::TokenStream;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::owned_weave::owned_weave;

pub(crate) struct ConsoleWeaveLedger {
    remaining: BTreeMap<usize, usize>,
}

impl ConsoleWeaveLedger {
    pub(crate) fn new(slot_uses: &[usize]) -> Self {
        let mut remaining: BTreeMap<usize, usize> = BTreeMap::new();

        for slot in slot_uses {
            *remaining.entry(*slot).or_insert(0) += 1;
        }

        Self { remaining }
    }

    pub(crate) fn weave(&mut self, argument: &ConsoleArgument, slot: usize) -> TokenStream {
        let remaining = self
            .remaining
            .get_mut(&slot)
            .expect("the ledger counts every slot it weaves");

        *remaining -= 1;

        owned_weave(argument, slot, *remaining == 0)
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_console_argument_codegen::weaving_kind::WeavingKind;

    use super::ConsoleWeaveLedger;

    fn collapsed(tokens: TokenStream) -> String {
        tokens.to_string().split_whitespace().collect()
    }

    fn cloned() -> ConsoleArgument {
        ConsoleArgument::Named {
            name: "mapper".to_string(),
            required: false,
            weaving: WeavingKind::Cloned,
            value_type: CanonicalPath::new(vec![
                "std".to_string(),
                "string".to_string(),
                "String".to_string(),
            ]),
            relations: Vec::new(),
        }
    }

    fn flag() -> ConsoleArgument {
        ConsoleArgument::Flag {
            name: "loud".to_string(),
        }
    }

    #[test]
    fn a_single_use_non_copy_slot_moves() {
        let mut ledger = ConsoleWeaveLedger::new(&[0]);

        assert_eq!(collapsed(ledger.weave(&cloned(), 0)), "console_argument_0");
    }

    #[test]
    fn a_twice_used_non_copy_slot_clones_then_moves() {
        let mut ledger = ConsoleWeaveLedger::new(&[0, 0]);

        assert_eq!(
            collapsed(ledger.weave(&cloned(), 0)),
            "console_argument_0.clone()"
        );
        assert_eq!(collapsed(ledger.weave(&cloned(), 0)), "console_argument_0");
    }

    #[test]
    fn a_thrice_used_non_copy_slot_clones_all_but_the_last() {
        let mut ledger = ConsoleWeaveLedger::new(&[0, 0, 0]);

        assert_eq!(
            collapsed(ledger.weave(&cloned(), 0)),
            "console_argument_0.clone()"
        );
        assert_eq!(
            collapsed(ledger.weave(&cloned(), 0)),
            "console_argument_0.clone()"
        );
        assert_eq!(collapsed(ledger.weave(&cloned(), 0)), "console_argument_0");
    }

    #[test]
    fn a_twice_used_copy_slot_never_clones() {
        let mut ledger = ConsoleWeaveLedger::new(&[1, 1]);

        assert_eq!(collapsed(ledger.weave(&flag(), 1)), "console_argument_1");
        assert_eq!(collapsed(ledger.weave(&flag(), 1)), "console_argument_1");
    }

    #[test]
    fn interleaved_slots_are_decided_independently() {
        let mut ledger = ConsoleWeaveLedger::new(&[0, 1, 0]);

        assert_eq!(
            collapsed(ledger.weave(&cloned(), 0)),
            "console_argument_0.clone()"
        );
        assert_eq!(collapsed(ledger.weave(&cloned(), 1)), "console_argument_1");
        assert_eq!(collapsed(ledger.weave(&cloned(), 0)), "console_argument_0");
    }
}
