use std::collections::BTreeMap;

use proc_macro2::TokenStream;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::owned_thread::owned_thread;

pub(crate) struct ConsoleThreadLedger {
    remaining: BTreeMap<usize, usize>,
}

impl ConsoleThreadLedger {
    pub(crate) fn new(slot_uses: &[usize]) -> Self {
        let mut remaining: BTreeMap<usize, usize> = BTreeMap::new();

        for slot in slot_uses {
            *remaining.entry(*slot).or_insert(0) += 1;
        }

        Self { remaining }
    }

    pub(crate) fn thread(&mut self, argument: &ConsoleArgument, slot: usize) -> TokenStream {
        let remaining = self
            .remaining
            .get_mut(&slot)
            .expect("the ledger counts every slot it threads");

        *remaining -= 1;

        owned_thread(argument, slot, *remaining == 0)
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_console_argument_codegen::threading_kind::ThreadingKind;

    use super::ConsoleThreadLedger;

    fn collapsed(tokens: TokenStream) -> String {
        tokens.to_string().split_whitespace().collect()
    }

    fn cloned() -> ConsoleArgument {
        ConsoleArgument::Named {
            name: "mapper".to_string(),
            required: false,
            threading: ThreadingKind::Cloned,
            value_type: CanonicalPath::new(vec![
                "std".to_string(),
                "string".to_string(),
                "String".to_string(),
            ]),
        }
    }

    fn flag() -> ConsoleArgument {
        ConsoleArgument::Flag {
            name: "loud".to_string(),
        }
    }

    #[test]
    fn a_single_use_non_copy_slot_moves() {
        let mut ledger = ConsoleThreadLedger::new(&[0]);

        assert_eq!(collapsed(ledger.thread(&cloned(), 0)), "console_argument_0");
    }

    #[test]
    fn a_twice_used_non_copy_slot_clones_then_moves() {
        let mut ledger = ConsoleThreadLedger::new(&[0, 0]);

        assert_eq!(
            collapsed(ledger.thread(&cloned(), 0)),
            "console_argument_0.clone()"
        );
        assert_eq!(collapsed(ledger.thread(&cloned(), 0)), "console_argument_0");
    }

    #[test]
    fn a_thrice_used_non_copy_slot_clones_all_but_the_last() {
        let mut ledger = ConsoleThreadLedger::new(&[0, 0, 0]);

        assert_eq!(
            collapsed(ledger.thread(&cloned(), 0)),
            "console_argument_0.clone()"
        );
        assert_eq!(
            collapsed(ledger.thread(&cloned(), 0)),
            "console_argument_0.clone()"
        );
        assert_eq!(collapsed(ledger.thread(&cloned(), 0)), "console_argument_0");
    }

    #[test]
    fn a_twice_used_copy_slot_never_clones() {
        let mut ledger = ConsoleThreadLedger::new(&[1, 1]);

        assert_eq!(collapsed(ledger.thread(&flag(), 1)), "console_argument_1");
        assert_eq!(collapsed(ledger.thread(&flag(), 1)), "console_argument_1");
    }

    #[test]
    fn interleaved_slots_are_decided_independently() {
        let mut ledger = ConsoleThreadLedger::new(&[0, 1, 0]);

        assert_eq!(
            collapsed(ledger.thread(&cloned(), 0)),
            "console_argument_0.clone()"
        );
        assert_eq!(collapsed(ledger.thread(&cloned(), 1)), "console_argument_1");
        assert_eq!(collapsed(ledger.thread(&cloned(), 0)), "console_argument_0");
    }
}
