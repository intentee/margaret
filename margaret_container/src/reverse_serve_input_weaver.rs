use std::collections::BTreeSet;

use proc_macro2::TokenStream;
use quote::quote;

use margaret_codegen_tokens::serve_input_ident::serve_input_ident;
use margaret_input_weaving::owned_weave::owned_weave;
use margaret_serve_input_codegen::serve_input::ServeInput;

pub(crate) struct ReverseServeInputWeaver {
    moved_slots: BTreeSet<usize>,
}

impl ReverseServeInputWeaver {
    pub(crate) fn new() -> Self {
        Self {
            moved_slots: BTreeSet::new(),
        }
    }

    pub(crate) fn weave(&mut self, input: &ServeInput, slot: usize) -> TokenStream {
        let is_last_use = self.moved_slots.insert(slot);
        let ident = serve_input_ident(slot);

        owned_weave(&input.weaving(), &quote! { #ident }, is_last_use)
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_console_argument_codegen::console_argument::ConsoleArgument;
    use margaret_input_weaving::input_value::InputValue;
    use margaret_input_weaving::weaving_kind::WeavingKind;
    use margaret_serve_input_codegen::serve_input::ServeInput;

    use super::ReverseServeInputWeaver;

    fn collapsed(tokens: &TokenStream) -> String {
        tokens.to_string().split_whitespace().collect()
    }

    fn cloned() -> ServeInput {
        ServeInput::ConsoleArgument(ConsoleArgument::Named {
            name: "mapper".to_string(),
            value: InputValue {
                required: false,
                value_type: CanonicalPath::new(vec![
                    "std".to_string(),
                    "string".to_string(),
                    "String".to_string(),
                ]),
                weaving: WeavingKind::Cloned,
            },
        })
    }

    fn flag() -> ServeInput {
        ServeInput::ConsoleArgument(ConsoleArgument::Flag {
            name: "loud".to_string(),
        })
    }

    #[test]
    fn moves_the_first_non_copy_slot_encountered_from_the_end() {
        let mut weaver = ReverseServeInputWeaver::new();

        assert_eq!(collapsed(&weaver.weave(&cloned(), 0)), "serve_input_0");
    }

    #[test]
    fn clones_earlier_non_copy_uses_after_the_last_use_has_been_seen() {
        let mut weaver = ReverseServeInputWeaver::new();

        assert_eq!(collapsed(&weaver.weave(&cloned(), 0)), "serve_input_0");
        assert_eq!(
            collapsed(&weaver.weave(&cloned(), 0)),
            "serve_input_0.clone()"
        );
        assert_eq!(
            collapsed(&weaver.weave(&cloned(), 0)),
            "serve_input_0.clone()"
        );
    }

    #[test]
    fn copy_slots_never_clone() {
        let mut weaver = ReverseServeInputWeaver::new();

        assert_eq!(collapsed(&weaver.weave(&flag(), 1)), "serve_input_1");
        assert_eq!(collapsed(&weaver.weave(&flag(), 1)), "serve_input_1");
    }

    #[test]
    fn slots_are_decided_independently() {
        let mut weaver = ReverseServeInputWeaver::new();

        assert_eq!(collapsed(&weaver.weave(&cloned(), 0)), "serve_input_0");
        assert_eq!(collapsed(&weaver.weave(&cloned(), 1)), "serve_input_1");
        assert_eq!(
            collapsed(&weaver.weave(&cloned(), 0)),
            "serve_input_0.clone()"
        );
    }
}
