use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::session_plan::SessionPlan;

pub(crate) fn render_server_routes(
    server: &str,
    sessions: &[&SessionPlan],
    has_views: bool,
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
    let views_name = if sessions
        .iter()
        .any(|session_plan| session_plan.session.injects_views())
    {
        format_ident!("views")
    } else {
        format_ident!("_views")
    };
    let views_parameter =
        has_views.then(|| quote! { #views_name: &::std::sync::Arc<super::views::Views>, });
    let entries = sessions.iter().map(|session_plan| {
        let path = &session_plan.session.path;
        let module = format_ident!("{}", session_plan.session.module_name);
        let routes_argument = session_plan
            .session
            .references_routes()
            .then(|| quote! { routes, });
        let views_argument = session_plan
            .session
            .injects_views()
            .then(|| quote! { views, });

        quote! {
            margaret_http::route_entry::RouteEntry::web_socket(
                #path,
                #module::upgrade_entry(container, #routes_argument #views_argument).await,
            ),
        }
    });

    quote! {
        pub async fn #function(
            container: &super::container::Container,
            #routes_name: &::std::sync::Arc<super::routes::Routes>,
            #views_parameter
        ) -> ::std::vec::Vec<margaret_http::route_entry::RouteEntry> {
            ::std::vec::Vec::from([#(#entries)*])
        }
    }
}
