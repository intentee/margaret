use std::collections::HashSet;

use quote::format_ident;
use quote::quote;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_codegen_tokens::synthetic_route::SyntheticRoute;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::emitted_message::EmittedMessage;
use crate::protocol::Protocol;
use crate::protocols::protocols;
use crate::render_handshake_handler::render_handshake_handler;
use crate::render_message_outbound::render_message_outbound;
use crate::render_protocol::render_protocol;
use crate::websocket_artifacts::WebsocketArtifacts;
use crate::websocket_codegen_error::WebsocketCodegenError;
use crate::websocket_internal_events::websocket_internal_events;
use crate::websocket_messages::websocket_messages;
use crate::websocket_states::websocket_states;
use crate::websocket_transitions::websocket_transitions;

fn distinct_emitted_messages<'model>(
    protocols: &[Protocol<'model>],
) -> Vec<&'model EmittedMessage> {
    let mut seen = HashSet::new();
    let mut messages = Vec::new();

    for protocol in protocols {
        for transition in &protocol.transitions {
            for emit in &transition.emits {
                if seen.insert(emit.canonical_path.clone()) {
                    messages.push(emit);
                }
            }
        }
    }

    messages
}

pub fn render_websocket(index: &AttributeIndex) -> Result<WebsocketArtifacts, WebsocketCodegenError> {
    let states = websocket_states(index)?;
    let messages = websocket_messages(index)?;
    let events = websocket_internal_events(index)?;
    let transitions = websocket_transitions(index, &states, &messages, &events)?;
    let grouped = protocols(&states, &transitions)?;

    let child_declarations = grouped.iter().map(|protocol| {
        let module = format_ident!("{}", protocol.entry_field);

        quote! { pub mod #module; }
    });

    let outbound_impls = distinct_emitted_messages(&grouped)
        .into_iter()
        .map(render_message_outbound);

    let mut modules = vec![GeneratedModuleTokens::new(
        "websocket",
        quote! {
            #(#child_declarations)*

            #(#outbound_impls)*
        },
    )];
    let mut synthetic_routes = Vec::new();

    for protocol in &grouped {
        modules.push(GeneratedModuleTokens::new(
            format!("websocket/{}", protocol.entry_field),
            render_protocol(protocol),
        ));

        synthetic_routes.push(SyntheticRoute {
            handler: render_handshake_handler(protocol),
            label: format!("websocket handshake for {}", protocol.path),
            path: protocol.path.clone(),
            server: protocol.server.clone(),
        });
    }

    Ok(WebsocketArtifacts {
        modules,
        synthetic_routes,
    })
}
