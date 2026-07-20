use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::session_plan::SessionPlan;

pub(crate) fn render_server_routes(server: &str, sessions: &[&SessionPlan]) -> TokenStream {
    let function = format_ident!("{server}_routes");
    let entries = sessions.iter().map(|session_plan| {
        let path = &session_plan.session.path;
        let module = format_ident!("{}", session_plan.session.module_name);

        quote! {
            margaret_http::route_entry::RouteEntry::web_socket(
                #path,
                #module::upgrade_entry(container).await,
            ),
        }
    });

    quote! {
        pub async fn #function(
            container: &super::container::Container,
        ) -> ::std::vec::Vec<margaret_http::route_entry::RouteEntry> {
            ::std::vec::Vec::from([#(#entries)*])
        }
    }
}
