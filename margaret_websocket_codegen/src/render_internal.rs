use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::path_tokens::path_tokens;

use crate::protocol::Protocol;
use crate::transition_trigger::TransitionTrigger;

pub(crate) fn internal_event_triggers<'protocol>(
    protocol: &'protocol Protocol<'_>,
) -> Vec<&'protocol TransitionTrigger> {
    let mut seen = HashSet::new();

    protocol
        .transitions
        .iter()
        .map(|transition| &transition.trigger)
        .filter(|trigger| matches!(trigger, TransitionTrigger::InternalEvent { .. }))
        .filter(|trigger| seen.insert(trigger.canonical_path().clone()))
        .collect()
}

pub(crate) fn render_internal(protocol: &Protocol<'_>) -> TokenStream {
    let triggers = internal_event_triggers(protocol);

    let variants = triggers.iter().map(|trigger| {
        let variant = format_ident!("{}", trigger.variant());
        let concrete = path_tokens(trigger.canonical_path());

        quote! { #variant(#concrete), }
    });

    let conversions = triggers.iter().map(|trigger| {
        let variant = format_ident!("{}", trigger.variant());
        let concrete = path_tokens(trigger.canonical_path());

        quote! {
            impl ::std::convert::From<#concrete> for Internal {
                fn from(event: #concrete) -> Self {
                    Internal::#variant(event)
                }
            }
        }
    });

    quote! {
        pub enum Internal {
            #(#variants)*
        }

        #(#conversions)*
    }
}
