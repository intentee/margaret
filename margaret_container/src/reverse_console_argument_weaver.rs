use std::collections::BTreeSet;

use proc_macro2::TokenStream;

use margaret_codegen_tokens::console_argument_field::console_argument_field;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_console_argument_codegen::owned_weave::owned_weave;

pub(crate) struct ReverseConsoleArgumentWeaver {
    moved_slots: BTreeSet<usize>,
}

impl ReverseConsoleArgumentWeaver {
    pub(crate) fn new() -> Self {
        Self {
            moved_slots: BTreeSet::new(),
        }
    }

    pub(crate) fn weave(&mut self, argument: &ConsoleArgument, slot: usize) -> TokenStream {
        let is_last_use = self.moved_slots.insert(slot);

        owned_weave(argument, &console_argument_field(slot), is_last_use)
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_console_argument_codegen::weaving_kind::WeavingKind;

    use super::ReverseConsoleArgumentWeaver;

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
        }
    }

    fn flag() -> ConsoleArgument {
        ConsoleArgument::Flag {
            name: "loud".to_string(),
        }
    }

    #[test]
    fn moves_the_first_non_copy_slot_encountered_from_the_end() {
        let mut weaver = ReverseConsoleArgumentWeaver::new();

        assert_eq!(collapsed(weaver.weave(&cloned(), 0)), "arguments.argument0");
    }

    #[test]
    fn clones_earlier_non_copy_uses_after_the_last_use_has_been_seen() {
        let mut weaver = ReverseConsoleArgumentWeaver::new();

        assert_eq!(collapsed(weaver.weave(&cloned(), 0)), "arguments.argument0");
        assert_eq!(
            collapsed(weaver.weave(&cloned(), 0)),
            "arguments.argument0.clone()"
        );
        assert_eq!(
            collapsed(weaver.weave(&cloned(), 0)),
            "arguments.argument0.clone()"
        );
    }

    #[test]
    fn copy_slots_never_clone() {
        let mut weaver = ReverseConsoleArgumentWeaver::new();

        assert_eq!(collapsed(weaver.weave(&flag(), 1)), "arguments.argument1");
        assert_eq!(collapsed(weaver.weave(&flag(), 1)), "arguments.argument1");
    }

    #[test]
    fn slots_are_decided_independently() {
        let mut weaver = ReverseConsoleArgumentWeaver::new();

        assert_eq!(collapsed(weaver.weave(&cloned(), 0)), "arguments.argument0");
        assert_eq!(collapsed(weaver.weave(&cloned(), 1)), "arguments.argument1");
        assert_eq!(
            collapsed(weaver.weave(&cloned(), 0)),
            "arguments.argument0.clone()"
        );
    }
}
