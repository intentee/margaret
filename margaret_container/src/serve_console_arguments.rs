use std::collections::BTreeMap;

use margaret_console_argument_codegen::console_argument::ConsoleArgument;

use crate::planned_provider::PlannedProvider;

pub(crate) struct ServeConsoleArguments {
    pub(crate) arguments: Vec<ConsoleArgument>,
    pub(crate) slots: Vec<usize>,
}

impl ServeConsoleArguments {
    pub(crate) fn from_roots(roots: &[&PlannedProvider]) -> Self {
        let mut arguments_by_slot = BTreeMap::new();

        for root in roots {
            for (argument, slot) in root
                .console_arguments
                .iter()
                .zip(root.console_slots.iter().copied())
            {
                arguments_by_slot.entry(slot).or_insert(argument);
            }
        }

        let mut arguments = Vec::with_capacity(arguments_by_slot.len());
        let mut slots = Vec::with_capacity(arguments_by_slot.len());

        for (slot, argument) in arguments_by_slot {
            arguments.push(argument.clone());
            slots.push(slot);
        }

        Self { arguments, slots }
    }
}
