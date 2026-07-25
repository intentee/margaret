use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_codegen_tokens::too_many_arguments_expect::too_many_arguments_expect;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_container::container_bindings::ContainerBindings;
use margaret_middleware_codegen::middleware_vec_tokens::middleware_vec_tokens;

use crate::session_console_arguments::session_console_arguments;
use crate::session_plan::SessionPlan;

pub(crate) fn server_console_arguments(
    sessions: &[&SessionPlan],
    bindings: &ContainerBindings,
) -> Vec<ConsoleArgument> {
    let mut collected: Vec<ConsoleArgument> = Vec::new();

    for session_plan in sessions {
        collected.extend(session_console_arguments(session_plan, bindings));

        for layer in &session_plan.session.layers {
            collected.extend_from_slice(bindings.console_arguments(&layer.concrete));
        }
    }

    bindings.console_union(&collected)
}

pub(crate) fn render_server_routes(
    server: &str,
    sessions: &[&SessionPlan],
    bindings: &ContainerBindings,
) -> TokenStream {
    let function = format_ident!("{server}_routes");
    let routes_name = if sessions
        .iter()
        .any(|session_plan| session_plan.session.references_routes())
    {
        format_ident!("routes")
    } else {
        format_ident!("_routes")
    };
    let console_parameters =
        bindings.console_parameters(&server_console_arguments(sessions, bindings));
    let entries = sessions.iter().map(|session_plan| {
        let path = &session_plan.session.path;
        let module = format_ident!("{}", session_plan.session.module_name);
        let routes_argument = session_plan
            .session
            .injects_routes()
            .then(|| quote! { routes, });
        let console_forward =
            bindings.console_forwards(&session_console_arguments(session_plan, bindings));
        let middleware = if session_plan.session.layers.is_empty() {
            quote! { ::std::vec::Vec::new() }
        } else {
            middleware_vec_tokens(
                &session_plan.session.layers,
                &quote! { super::middleware },
                bindings,
            )
        };

        quote! {
            margaret::framework::http::route_entry::RouteEntry::web_socket(
                #path,
                #module::upgrade_entry(container, #(#console_forward)* #routes_argument).await,
                #middleware,
            ),
        }
    });

    let parameter_count = 2 + console_parameters.len();
    let too_many_arguments = too_many_arguments_expect(parameter_count);

    quote! {
        #too_many_arguments
        pub async fn #function(
            container: &super::container::Container,
            #(#console_parameters)*
            #routes_name: &::std::sync::Arc<super::routes::Routes>,
        ) -> ::std::vec::Vec<margaret::framework::http::route_entry::RouteEntry> {
            ::std::vec::Vec::from([#(#entries)*])
        }
    }
}
