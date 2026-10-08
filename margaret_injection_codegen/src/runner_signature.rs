use proc_macro2::Ident;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;

use crate::is_cancellation_token::is_cancellation_token;
use crate::parameters::parameters;
use crate::request_binding_marker::request_binding_marker;

pub enum RunnerSignature<'runner> {
    Accepted {
        takes_token: bool,
    },
    RejectedArgument {
        parameter: &'runner Ident,
    },
    RejectedRequestBinding {
        marker: FrameworkAttribute,
        parameter: &'runner Ident,
    },
}

impl<'runner> RunnerSignature<'runner> {
    #[must_use]
    pub fn of(index: &AttributeIndex, item: &IndexedItem, runner: &'runner IndexedMethod) -> Self {
        let mut takes_token = false;

        for view in parameters(runner) {
            if let Some(marker) = request_binding_marker(view.attributes) {
                return Self::RejectedRequestBinding {
                    marker,
                    parameter: view.name,
                };
            }

            if !is_cancellation_token(index, item, view.declared) {
                return Self::RejectedArgument {
                    parameter: view.name,
                };
            }

            takes_token = true;
        }

        Self::Accepted { takes_token }
    }
}
