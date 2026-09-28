use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_container::container_bindings::ContainerBindings;

use crate::authenticated_user_application::AuthenticatedUserApplication;
use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
use crate::bearer_token_verifier_field::BEARER_TOKEN_VERIFIER_FIELD;

#[must_use]
pub fn render_authenticated_user_wrapper_construction(
    AuthenticatedUserApplication {
        challenge,
        field,
        injects_routes,
        injects_views,
        wrapper,
        ..
    }: &AuthenticatedUserApplication,
    authenticated_users_module: &TokenStream,
    container: &Ident,
    bindings: &ContainerBindings,
) -> TokenStream {
    let inner = bindings.accessor_invocation(container, field);
    let routes_init = injects_routes.then(|| quote! { routes: routes.clone(), });
    let views_init = injects_views.then(|| quote! { views: views.clone(), });
    let verifier_init = match challenge {
        AuthenticatedUserChallenge::Bearer { issuer_client } => {
            let verifier_field = format_ident!("{BEARER_TOKEN_VERIFIER_FIELD}");
            let client = bindings.accessor_invocation(container, &issuer_client.field);

            quote! { #verifier_field: #client.verifier(), }
        }
        AuthenticatedUserChallenge::Unchallenged => TokenStream::new(),
    };

    quote! {
        ::std::sync::Arc::new(
            #authenticated_users_module::#wrapper {
                inner: #inner,
                #routes_init
                #views_init
                #verifier_init
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use quote::format_ident;
    use quote::quote;

    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::injected_dependency::InjectedDependency;
    use margaret_container::render_container::render_container;
    use margaret_serve_input_codegen::scan::scan;

    use super::render_authenticated_user_wrapper_construction;
    use crate::authenticated_user_application::AuthenticatedUserApplication;
    use crate::authenticated_user_challenge::AuthenticatedUserChallenge;

    fn path(name: &str) -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), name.to_string()])
    }

    fn application(
        challenge: AuthenticatedUserChallenge,
        injects_routes: bool,
        injects_views: bool,
    ) -> AuthenticatedUserApplication {
        AuthenticatedUserApplication {
            challenge,
            concrete: path("Provider"),
            field: "provider".to_string(),
            injects_peer_spiffe_id: false,
            injects_routes,
            injects_views,
            model: path("User"),
            wrapper: format_ident!("Provider"),
        }
    }

    fn bindings() -> ContainerBindings {
        let index = IndexedSource::new("#[singleton]\nstruct Config;\n").index;

        render_container(
            &index,
            &scan(&index).expect("the serve inputs are scanned"),
            &[],
        )
        .expect("the container renders")
        .bindings
    }

    fn construction(application: &AuthenticatedUserApplication) -> String {
        render_authenticated_user_wrapper_construction(
            application,
            &quote! { super::authenticated_users },
            &format_ident!("container"),
            &bindings(),
        )
        .to_string()
        .replace(' ', "")
    }

    #[test]
    fn hands_the_wrapper_only_its_inner_provider() {
        assert_eq!(
            construction(&application(
                AuthenticatedUserChallenge::Unchallenged,
                false,
                false
            )),
            "::std::sync::Arc::new(super::authenticated_users::Provider{inner:container.provider(),},)"
        );
    }

    #[test]
    fn hands_the_wrapper_the_routes_and_views_its_provider_reads() {
        assert_eq!(
            construction(&application(
                AuthenticatedUserChallenge::Unchallenged,
                true,
                true
            )),
            "::std::sync::Arc::new(super::authenticated_users::Provider{inner:container.provider(),routes:routes.clone(),views:views.clone(),},)"
        );
    }

    #[test]
    fn hands_the_wrapper_the_verifier_of_the_token_issuer_its_provider_reads() {
        assert_eq!(
            construction(&application(
                AuthenticatedUserChallenge::Bearer {
                    issuer_client: InjectedDependency {
                        concrete: path("Client"),
                        field: "partner_client".to_string(),
                    },
                },
                false,
                false,
            )),
            "::std::sync::Arc::new(super::authenticated_users::Provider{inner:container.provider(),bearer_token_verifier:container.partner_client().verifier(),},)"
        );
    }
}
