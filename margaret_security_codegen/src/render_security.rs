use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::path_tokens::path_tokens;

use crate::build_security_plan::build_security_plan;
use crate::gatekeeper_provider::synthetic_providers;
use crate::security_artifacts::SecurityArtifacts;
use crate::security_codegen_error::SecurityCodegenError;
use crate::security_plan::SecurityPlan;

struct Component {
    field: Ident,
    type_tokens: TokenStream,
}

fn component(path: &CanonicalPath) -> Component {
    Component {
        field: format_ident!("{}", path.field_name()),
        type_tokens: path_tokens(path),
    }
}

fn components(plan: &SecurityPlan) -> Vec<Component> {
    let mut components = vec![component(&plan.store_path)];

    for gate in &plan.crud_gates {
        components.push(component(&gate.gate_path));
    }

    for gate in &plan.site_gates {
        components.push(component(&gate.gate_path));
    }

    components
}

fn imports(plan: &SecurityPlan) -> TokenStream {
    let crud_imports = if plan.crud_gates.is_empty() {
        quote! {}
    } else {
        quote! {
            use margaret_security::crud_action::CrudAction;
            use margaret_security::crud_action_gate::CrudActionGate;
            use margaret_security::crud_action_gate_registry::CrudActionGateRegistry;
        }
    };
    let site_imports = if plan.site_gates.is_empty() {
        quote! {}
    } else {
        quote! {
            use margaret_security::site_action_dispatcher::SiteActionDispatcher;
            use margaret_security::site_action_gate::SiteActionGate;
        }
    };

    quote! {
        use margaret_security::authenticated_actor_store::AuthenticatedActorStore;
        use margaret_security::authenticated_actor::AuthenticatedActor;
        use margaret_security::gatekeeper_backend::GatekeeperBackend;
        #crud_imports
        #site_imports
    }
}

fn structure(components: &[Component]) -> TokenStream {
    let fields = components.iter().map(|Component { field, type_tokens }| {
        quote! { #field: std::sync::Arc<#type_tokens> }
    });

    quote! {
        pub struct SecurityBackend {
            #(#fields,)*
        }
    }
}

fn constructor(components: &[Component]) -> TokenStream {
    let parameters = components.iter().map(|Component { field, type_tokens }| {
        quote! { #field: std::sync::Arc<#type_tokens> }
    });
    let fields = components
        .iter()
        .map(|Component { field, .. }| quote! { #field });

    quote! {
        impl SecurityBackend {
            pub fn new(#(#parameters),*) -> Self {
                Self {
                    #(#fields,)*
                }
            }
        }
    }
}

fn backend_impl(plan: &SecurityPlan) -> TokenStream {
    let store_field = format_ident!("{}", plan.store_path.field_name());
    let user = path_tokens(&plan.actor_type);

    quote! {
        #[async_trait::async_trait]
        impl GatekeeperBackend for SecurityBackend {
            type Actor = #user;

            async fn authenticate(
                &self,
                request: &margaret_http::request::Request,
            ) -> AuthenticatedActor<#user> {
                self.#store_field.get_authenticated_actor(request).await
            }
        }
    }
}

fn dispatcher_impl(plan: &SecurityPlan) -> TokenStream {
    let Some(action_type) = &plan.action_type else {
        return quote! {};
    };

    let user = path_tokens(&plan.actor_type);
    let arms = plan.site_gates.iter().map(|gate| {
        let action_path = &gate.action_path;
        let field = format_ident!("{}", gate.gate_path.field_name());

        quote! { #action_path => self.#field.can(authenticated_actor).await, }
    });

    quote! {
        #[async_trait::async_trait]
        impl SiteActionDispatcher<#user> for SecurityBackend {
            type SiteAction = #action_type;

            async fn can_site_action(
                &self,
                authenticated_actor: &AuthenticatedActor<#user>,
                action: #action_type,
            ) -> bool {
                match action {
                    #(#arms)*
                }
            }
        }
    }
}

fn registry_impls(plan: &SecurityPlan) -> TokenStream {
    let user = path_tokens(&plan.actor_type);
    let impls = plan.crud_gates.iter().map(|gate| {
        let subject = path_tokens(&gate.subject_type);
        let field = format_ident!("{}", gate.gate_path.field_name());

        quote! {
            #[async_trait::async_trait]
            impl CrudActionGateRegistry<#user, #subject> for SecurityBackend {
                async fn can_crud(
                    &self,
                    authenticated_actor: &AuthenticatedActor<#user>,
                    subject: &#subject,
                    action: CrudAction,
                ) -> bool {
                    self.#field.can(authenticated_actor, subject, action).await
                }
            }
        }
    });

    quote! { #(#impls)* }
}

fn render(plan: &SecurityPlan) -> String {
    let components = components(plan);
    let imports = imports(plan);
    let structure = structure(&components);
    let constructor = constructor(&components);
    let backend_impl = backend_impl(plan);
    let dispatcher_impl = dispatcher_impl(plan);
    let registry_impls = registry_impls(plan);

    let tokens = quote! {
        #imports

        #structure

        #constructor

        #backend_impl

        #dispatcher_impl

        #registry_impls

        pub type Gatekeeper = margaret_security::gatekeeper::Gatekeeper<SecurityBackend>;
    };

    let file =
        syn::parse2::<syn::File>(tokens).expect("the generated tokens form a valid Rust file");

    prettyplease::unparse(&file)
}

pub fn render_security(
    index: &AttributeIndex,
) -> Result<Option<SecurityArtifacts>, SecurityCodegenError> {
    let Some(plan) = build_security_plan(index)? else {
        return Ok(None);
    };

    Ok(Some(SecurityArtifacts {
        module_source: render(&plan),
        providers: synthetic_providers(&plan),
    }))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;

    use super::render_security;

    const MODELS_AND_STORE: &str = r#"
struct User;
struct Article;

#[provides_authenticated_actor]
struct SessionStore;

impl AuthenticatedActorStore for SessionStore {
    type Actor = User;
    async fn get_authenticated_actor(&self, request: &Request) -> AuthenticatedActor<User> {}
}
"#;

    const ARTICLE_GATE: &str = r#"
#[decides_crud_action]
struct ArticleGate;

impl CrudActionGate for ArticleGate {
    type Actor = User;
    type Subject = Article;
    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>, subject: &Article, action: CrudAction) -> bool {}
}
"#;

    const MANAGE_USERS_GATE: &str = r#"
#[decides_site_action(crate::action::Action::ManageUsers)]
struct ManageUsersGate;

impl SiteActionGate for ManageUsersGate {
    type Actor = User;
    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>) -> bool {}
}
"#;

    fn index_for(source: &str) -> AttributeIndex {
        let directory = tempdir().expect("a temporary crate directory");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), source).expect("lib.rs is written");

        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", &source_directory))
            .expect("the crate is indexed")
            .build()
    }

    fn source_for(parts: &[&str]) -> String {
        let index = index_for(&parts.concat());

        render_security(&index)
            .expect("the security source is generated")
            .expect("a security plan is present")
            .module_source
            .split_whitespace()
            .collect()
    }

    fn error_for(source: &str) -> String {
        render_security(&index_for(source))
            .err()
            .expect("the security source must fail to generate")
            .to_string()
    }

    #[test]
    fn wires_the_store_into_the_security_backend() {
        let source = source_for(&[MODELS_AND_STORE, ARTICLE_GATE, MANAGE_USERS_GATE]);

        assert!(source.contains("pubstructSecurityBackend"));
        assert!(source.contains("session_store:std::sync::Arc<crate::SessionStore>"));
        assert!(
            source.contains("#[async_trait::async_trait]implGatekeeperBackendforSecurityBackend")
        );
        assert!(source.contains("self.session_store.get_authenticated_actor(request).await"));
    }

    #[test]
    fn aliases_the_gatekeeper_to_the_framework_type() {
        let source = source_for(&[MODELS_AND_STORE, ARTICLE_GATE, MANAGE_USERS_GATE]);

        assert!(source.contains(
            "pubtypeGatekeeper=margaret_security::gatekeeper::Gatekeeper<SecurityBackend>;"
        ));
        assert!(!source.contains("pubstructGatekeeper"));
        assert!(!source.contains("GatekeeperUserContext"));
        assert!(!source.contains("with_request"));
        assert!(!source.contains("can_crud_all"));
    }

    #[test]
    fn dispatches_crud_actions_through_a_typed_registry() {
        let source = source_for(&[MODELS_AND_STORE, ARTICLE_GATE, MANAGE_USERS_GATE]);

        assert!(
            source.contains(
                "#[async_trait::async_trait]implCrudActionGateRegistry<crate::User,crate::Article>forSecurityBackend"
            )
        );
        assert!(source.contains("self.article_gate.can(authenticated_actor,subject,action).await"));
        assert!(source.contains("usemargaret_security::crud_action_gate::CrudActionGate;"));
    }

    #[test]
    fn dispatches_site_actions_through_a_match() {
        let source = source_for(&[MODELS_AND_STORE, ARTICLE_GATE, MANAGE_USERS_GATE]);

        assert!(source.contains(
            "#[async_trait::async_trait]implSiteActionDispatcher<crate::User>forSecurityBackend"
        ));
        assert!(source.contains("typeSiteAction=crate::action::Action;"));
        assert!(source.contains(
            "crate::action::Action::ManageUsers=>{self.manage_users_gate.can(authenticated_actor).await}"
        ));
        assert!(source.contains("usemargaret_security::site_action_gate::SiteActionGate;"));
    }

    #[test]
    fn wires_multiple_crud_and_site_gates_in_a_stable_order() {
        let source: String = {
            let index = index_for(
                r#"
struct User;
struct Article;
struct Comment;

#[provides_authenticated_actor]
struct SessionStore;

impl AuthenticatedActorStore for SessionStore {
    type Actor = User;
    async fn get_authenticated_actor(&self, request: &Request) -> AuthenticatedActor<User> {}
}

#[decides_crud_action]
struct CommentGate;

impl CrudActionGate for CommentGate {
    type Actor = User;
    type Subject = Comment;
    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>, subject: &Comment, action: CrudAction) -> bool {}
}

#[decides_crud_action]
struct ArticleGate;

impl CrudActionGate for ArticleGate {
    type Actor = User;
    type Subject = Article;
    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>, subject: &Article, action: CrudAction) -> bool {}
}

#[decides_site_action(crate::action::Action::ViewReports)]
struct ViewReportsGate;

impl SiteActionGate for ViewReportsGate {
    type Actor = User;
    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>) -> bool {}
}

#[decides_site_action(crate::action::Action::ManageUsers)]
struct ManageUsersGate;

impl SiteActionGate for ManageUsersGate {
    type Actor = User;
    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>) -> bool {}
}
"#,
            );

            render_security(&index)
                .expect("the security source is generated")
                .expect("a security plan is present")
                .module_source
                .split_whitespace()
                .collect()
        };

        assert!(
            source.contains(
                "implCrudActionGateRegistry<crate::User,crate::Article>forSecurityBackend"
            )
        );
        assert!(
            source.contains(
                "implCrudActionGateRegistry<crate::User,crate::Comment>forSecurityBackend"
            )
        );
        assert!(source.contains("crate::action::Action::ManageUsers=>"));
        assert!(source.contains("crate::action::Action::ViewReports=>"));

        let article = source
            .find("article_gate:std::sync::Arc")
            .expect("the article gate is a field");
        let comment = source
            .find("comment_gate:std::sync::Arc")
            .expect("the comment gate is a field");

        assert!(article < comment);
    }

    #[test]
    fn omits_site_action_dispatch_without_site_gates() {
        let source = source_for(&[MODELS_AND_STORE, ARTICLE_GATE]);

        assert!(!source.contains("SiteActionDispatcher"));
        assert!(!source.contains("can_site_action"));
        assert!(source.contains("implCrudActionGateRegistry"));
    }

    #[test]
    fn omits_crud_dispatch_without_crud_gates() {
        let source = source_for(&[MODELS_AND_STORE, MANAGE_USERS_GATE]);

        assert!(!source.contains("implCrudActionGateRegistry<"));
        assert!(!source.contains("usemargaret_security::crud_action_gate::CrudActionGate;"));
        assert!(source.contains("implSiteActionDispatcher<crate::User>forSecurityBackend"));
    }

    #[test]
    fn generates_a_login_only_security_backend() {
        let source = source_for(&[MODELS_AND_STORE]);

        assert!(source.contains("pubstructSecurityBackend"));
        assert!(source.contains("implGatekeeperBackendforSecurityBackend"));
        assert!(!source.contains("SiteActionDispatcher"));
        assert!(!source.contains("implCrudActionGateRegistry<"));
        assert!(source.contains(
            "pubtypeGatekeeper=margaret_security::gatekeeper::Gatekeeper<SecurityBackend>;"
        ));
    }

    #[test]
    fn rejects_two_authenticated_actor_stores() {
        let source = format!(
            "{MODELS_AND_STORE}\n#[provides_authenticated_actor]\nstruct OtherStore;\n\nimpl AuthenticatedActorStore for OtherStore {{\n    type Actor = User;\n    async fn get_authenticated_actor(&self, request: &Request) -> AuthenticatedActor<User> {{}}\n}}\n"
        );

        assert!(error_for(&source).contains("more than one #[provides_authenticated_actor]"));
    }

    #[test]
    fn rejects_a_store_without_a_actor_type() {
        let source = "#[provides_authenticated_actor]\nstruct SessionStore;\n";

        assert!(
            error_for(source)
                .contains("store 'crate::SessionStore' has no `type Actor = <struct>`")
        );
    }

    #[test]
    fn rejects_a_crud_gate_without_a_actor_type() {
        let source = format!(
            "{MODELS_AND_STORE}\n#[decides_crud_action]\nstruct ArticleGate;\n\nimpl CrudActionGate for ArticleGate {{\n    type Subject = Article;\n    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>, subject: &Article, action: CrudAction) -> bool {{}}\n}}\n"
        );

        assert!(
            error_for(&source).contains("gate 'crate::ArticleGate' has no `type Actor = <struct>`")
        );
    }

    #[test]
    fn rejects_a_crud_gate_for_a_foreign_user() {
        let source = format!(
            "{MODELS_AND_STORE}\nstruct Robot;\n\n#[decides_crud_action]\nstruct ArticleGate;\n\nimpl CrudActionGate for ArticleGate {{\n    type Actor = Robot;\n    type Subject = Article;\n    async fn can(&self, authenticated_actor: &AuthenticatedActor<Robot>, subject: &Article, action: CrudAction) -> bool {{}}\n}}\n"
        );

        assert!(
            error_for(&source)
                .contains("gate 'crate::ArticleGate' decides for user 'crate::Robot'")
        );
    }

    #[test]
    fn rejects_a_crud_gate_without_a_subject_type() {
        let source = format!(
            "{MODELS_AND_STORE}\n#[decides_crud_action]\nstruct ArticleGate;\n\nimpl CrudActionGate for ArticleGate {{\n    type Actor = User;\n    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>, action: CrudAction) -> bool {{}}\n}}\n"
        );

        assert!(
            error_for(&source)
                .contains("gate 'crate::ArticleGate' has no `type Subject = <struct>`")
        );
    }

    #[test]
    fn rejects_two_crud_gates_for_one_subject() {
        let source = format!(
            "{MODELS_AND_STORE}{ARTICLE_GATE}\n#[decides_crud_action]\nstruct OtherArticleGate;\n\nimpl CrudActionGate for OtherArticleGate {{\n    type Actor = User;\n    type Subject = Article;\n    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>, subject: &Article, action: CrudAction) -> bool {{}}\n}}\n"
        );

        assert!(error_for(&source).contains("has more than one CRUD gate"));
    }

    #[test]
    fn rejects_a_site_gate_without_a_actor_type() {
        let source = format!(
            "{MODELS_AND_STORE}\n#[decides_site_action(crate::action::Action::ManageUsers)]\nstruct ManageUsersGate;\n\nimpl SiteActionGate for ManageUsersGate {{\n    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>) -> bool {{}}\n}}\n"
        );

        assert!(
            error_for(&source)
                .contains("gate 'crate::ManageUsersGate' has no `type Actor = <struct>`")
        );
    }

    #[test]
    fn rejects_a_site_gate_for_a_foreign_user() {
        let source = format!(
            "{MODELS_AND_STORE}\nstruct Robot;\n\n#[decides_site_action(crate::action::Action::ManageUsers)]\nstruct ManageUsersGate;\n\nimpl SiteActionGate for ManageUsersGate {{\n    type Actor = Robot;\n    async fn can(&self, authenticated_actor: &AuthenticatedActor<Robot>) -> bool {{}}\n}}\n"
        );

        assert!(
            error_for(&source)
                .contains("gate 'crate::ManageUsersGate' decides for user 'crate::Robot'")
        );
    }

    #[test]
    fn rejects_a_site_gate_without_an_action_argument() {
        let source = format!(
            "{MODELS_AND_STORE}\n#[decides_site_action]\nstruct ManageUsersGate;\n\nimpl SiteActionGate for ManageUsersGate {{\n    type Actor = User;\n    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>) -> bool {{}}\n}}\n"
        );

        assert!(error_for(&source).contains("is missing its site action argument"));
    }

    #[test]
    fn propagates_malformed_site_action_arguments() {
        let source = format!(
            "{MODELS_AND_STORE}\n#[decides_site_action(= 5)]\nstruct ManageUsersGate;\n\nimpl SiteActionGate for ManageUsersGate {{\n    type Actor = User;\n    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>) -> bool {{}}\n}}\n"
        );

        assert!(error_for(&source).contains("failed to index"));
    }

    #[test]
    fn rejects_a_site_action_without_an_enum_qualifier() {
        let source = format!(
            "{MODELS_AND_STORE}\n#[decides_site_action(ManageUsers)]\nstruct ManageUsersGate;\n\nimpl SiteActionGate for ManageUsersGate {{\n    type Actor = User;\n    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>) -> bool {{}}\n}}\n"
        );

        assert!(error_for(&source).contains("is not a fully qualified"));
    }

    #[test]
    fn rejects_two_gates_for_one_site_action() {
        let source = format!(
            "{MODELS_AND_STORE}{MANAGE_USERS_GATE}\n#[decides_site_action(crate::action::Action::ManageUsers)]\nstruct OtherManageUsersGate;\n\nimpl SiteActionGate for OtherManageUsersGate {{\n    type Actor = User;\n    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>) -> bool {{}}\n}}\n"
        );

        assert!(error_for(&source).contains("has more than one gate"));
    }

    #[test]
    fn rejects_site_actions_from_different_enums() {
        let source = format!(
            "{MODELS_AND_STORE}{MANAGE_USERS_GATE}\n#[decides_site_action(crate::area::Area::ViewReports)]\nstruct ViewReportsGate;\n\nimpl SiteActionGate for ViewReportsGate {{\n    type Actor = User;\n    async fn can(&self, authenticated_actor: &AuthenticatedActor<User>) -> bool {{}}\n}}\n"
        );

        assert!(error_for(&source).contains("all site actions must share one enum"));
    }

    #[test]
    fn describes_the_backend_and_gatekeeper_providers() {
        let index = index_for(&[MODELS_AND_STORE, ARTICLE_GATE, MANAGE_USERS_GATE].concat());
        let providers = render_security(&index)
            .expect("the security artifacts are generated")
            .expect("a security plan is present")
            .providers;

        assert_eq!(providers.len(), 2);

        let backend = &providers[0];

        assert_eq!(
            backend.concrete_path.to_string(),
            "crate::margaret::security::SecurityBackend"
        );
        assert_eq!(backend.constructor, "new");
        assert_eq!(backend.field_name, "security_backend");

        let backend_dependencies: Vec<String> = backend
            .dependencies
            .iter()
            .map(|dependency| dependency.to_string())
            .collect();

        assert_eq!(
            backend_dependencies,
            vec![
                "crate::SessionStore".to_string(),
                "crate::ArticleGate".to_string(),
                "crate::ManageUsersGate".to_string(),
            ]
        );

        let gatekeeper = &providers[1];

        assert_eq!(
            gatekeeper.concrete_path.to_string(),
            "crate::margaret::security::Gatekeeper"
        );
        assert_eq!(gatekeeper.field_name, "gatekeeper");

        let gatekeeper_dependencies: Vec<String> = gatekeeper
            .dependencies
            .iter()
            .map(|dependency| dependency.to_string())
            .collect();

        assert_eq!(
            gatekeeper_dependencies,
            vec!["crate::margaret::security::SecurityBackend".to_string()]
        );
    }

    #[test]
    fn yields_no_artifacts_without_an_authenticated_actor_store() {
        let index = index_for("struct User;\n");

        assert!(
            render_security(&index)
                .expect("the security artifacts are computed")
                .is_none()
        );
    }
}
