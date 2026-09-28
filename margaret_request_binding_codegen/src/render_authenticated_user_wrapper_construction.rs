use proc_macro2::TokenStream;
use quote::quote;

use crate::authenticated_user_application::AuthenticatedUserApplication;

#[must_use]
pub fn render_authenticated_user_wrapper_construction(
    AuthenticatedUserApplication {
        injects_routes,
        injects_views,
        wrapper,
        ..
    }: &AuthenticatedUserApplication,
    authenticated_users_module: &TokenStream,
    inner: &TokenStream,
) -> TokenStream {
    let routes_init = injects_routes.then(|| quote! { routes: routes.clone(), });
    let views_init = injects_views.then(|| quote! { views: views.clone(), });

    quote! {
        ::std::sync::Arc::new(
            #authenticated_users_module::#wrapper {
                inner: #inner,
                #routes_init
                #views_init
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use quote::format_ident;
    use quote::quote;

    use margaret_attributes::canonical_path::CanonicalPath;

    use super::render_authenticated_user_wrapper_construction;
    use crate::authenticated_user_application::AuthenticatedUserApplication;

    fn application(injects_routes: bool, injects_views: bool) -> AuthenticatedUserApplication {
        AuthenticatedUserApplication {
            concrete: CanonicalPath::new(vec!["crate".to_string(), "Provider".to_string()]),
            field: "provider".to_string(),
            injects_peer_spiffe_id: false,
            injects_routes,
            injects_views,
            model: CanonicalPath::new(vec!["crate".to_string(), "User".to_string()]),
            wrapper: format_ident!("Provider"),
        }
    }

    fn construction(application: &AuthenticatedUserApplication) -> String {
        render_authenticated_user_wrapper_construction(
            application,
            &quote! { super::authenticated_users },
            &quote! { container.provider() },
        )
        .to_string()
        .replace(' ', "")
    }

    #[test]
    fn hands_the_wrapper_only_its_inner_provider() {
        assert_eq!(
            construction(&application(false, false)),
            "::std::sync::Arc::new(super::authenticated_users::Provider{inner:container.provider(),},)"
        );
    }

    #[test]
    fn hands_the_wrapper_the_routes_and_views_its_provider_reads() {
        assert_eq!(
            construction(&application(true, true)),
            "::std::sync::Arc::new(super::authenticated_users::Provider{inner:container.provider(),routes:routes.clone(),views:views.clone(),},)"
        );
    }
}
