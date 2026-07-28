pub mod authenticated_user_application;
pub mod authenticated_user_provider;
pub mod authenticated_user_providers;
pub mod authenticated_user_requirement;
pub mod binding_console_arguments;
pub mod binding_context;
pub mod binding_reads_request;
pub mod binding_registries;
pub mod binding_shadows_request;
pub mod bound_parameter;
pub mod captured_provider;
pub mod captured_provider_kind;
pub mod captured_providers;
pub mod classify_parameters;
pub mod extraction_context;
mod form_request_arguments;
pub mod form_request_extraction;
mod infers_authenticated_user_arguments;
pub mod injects_routes;
pub mod injects_views;
pub mod observes_cancellation;
pub mod render_authenticated_user_wrappers;
pub mod render_bound_request_extractions;
pub mod render_request_extraction;
pub mod request_binding;
pub mod request_binding_error;
pub mod request_injectable;
pub mod request_input_source;
mod route_parameter_arguments;
pub mod route_parameter_binder;
pub mod route_parameter_binders;
pub mod views_availability;

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::crate_root::CrateRoot;
    use margaret_console_argument_codegen::scan::scan;
    use margaret_container::injected_dependency::InjectedDependency;
    use margaret_container::render_container::render_container;
    use margaret_injection_codegen::process_method::process_method;
    use margaret_route_parameter_codegen::route_path::RoutePath;

    use crate::authenticated_user_application::AuthenticatedUserApplication;
    use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
    use crate::binding_console_arguments::binding_console_arguments;
    use crate::binding_context::BindingContext;
    use crate::binding_registries::BindingRegistries;
    use crate::bound_parameter::BoundParameter;
    use crate::classify_parameters::classify_parameters;
    use crate::render_authenticated_user_wrappers::render_authenticated_user_wrappers;
    use crate::request_binding::RequestBinding;
    use crate::request_binding_error::RequestBindingError;
    use crate::views_availability::ViewsAvailability;
    use quote::format_ident;

    const PRELUDE: &str = "\
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

struct User;
";

    fn index_for(lib_source: &str) -> AttributeIndex {
        let directory = tempdir().expect("a temporary crate directory is created");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), lib_source).expect("lib.rs is written");

        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", source_directory))
            .expect("the crate is indexed")
            .build()
    }

    fn provider_source(declaration: &str) -> String {
        format!("{PRELUDE}{declaration}")
    }

    fn empty_bindings() -> margaret_container::container_bindings::ContainerBindings {
        let index = index_for("");
        let registry = scan(&index).expect("the empty console argument registry is scanned");

        render_container(&index, &registry, &[])
            .expect("the empty container is rendered")
            .bindings
    }

    fn missing_path() -> margaret_attributes::canonical_path::CanonicalPath {
        margaret_attributes::canonical_path::CanonicalPath::new(vec![
            "crate".to_string(),
            "Missing".to_string(),
        ])
    }

    fn registries_with(declaration: &str, views: ViewsAvailability) -> BindingRegistries {
        BindingRegistries::collect(&index_for(&provider_source(declaration)), views)
            .expect("the binding registries are collected")
    }

    fn registries_for(declaration: &str) -> BindingRegistries {
        registries_with(declaration, ViewsAvailability::Available)
    }

    fn rejection_for(declaration: &str) -> String {
        BindingRegistries::collect(
            &index_for(&provider_source(declaration)),
            ViewsAvailability::Available,
        )
        .err()
        .expect("the binding registries are rejected")
        .to_string()
    }

    const SESSION_PROVIDER: &str = "\
#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[infer_from_request]
    fn infer(&self, request: &Request) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}
";

    #[test]
    fn registers_a_provider_under_the_user_model_it_infers() {
        let registries = registries_for(SESSION_PROVIDER);
        let providers = registries.providers();

        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].application.model.to_string(), "crate::User");
        assert_eq!(providers[0].application.field, "session_user_provider");
        assert_eq!(
            providers[0].application.wrapper.to_string(),
            "SessionUserProvider"
        );
        assert_eq!(providers[0].method_name.to_string(), "infer");
    }

    #[test]
    fn rejects_a_provider_on_a_non_struct() {
        assert!(
            rejection_for(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nenum Bad {}\n"
            )
            .contains("is only supported on structs")
        );
    }

    #[test]
    fn rejects_a_repeated_provider_attribute() {
        assert!(
            rejection_for(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\n#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n"
            )
            .contains("is repeated on")
        );
    }

    #[test]
    fn rejects_a_provider_that_is_not_a_singleton() {
        assert!(
            rejection_for("#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n")
                .contains("must also be declared as a #[singleton]")
        );
    }

    #[test]
    fn rejects_a_provider_without_a_user_model() {
        assert!(
            rejection_for("#[singleton]\n#[infers_authenticated_user]\nstruct Bad;\n")
                .contains("is missing `user_model")
        );
    }

    #[test]
    fn rejects_a_provider_whose_user_model_is_not_a_path() {
        assert!(
            rejection_for(
                "#[singleton]\n#[infers_authenticated_user(user_model = \"User\")]\nstruct Bad;\n"
            )
            .contains("is not a path")
        );
    }

    #[test]
    fn rejects_a_provider_whose_user_model_matches_no_struct() {
        assert!(
            rejection_for(
                "#[singleton]\n#[infers_authenticated_user(user_model = Ghost)]\nstruct Bad;\n"
            )
            .contains("matches no struct")
        );
    }

    #[test]
    fn propagates_malformed_provider_arguments() {
        assert!(
            rejection_for("#[singleton]\n#[infers_authenticated_user(= 5)]\nstruct Bad;\n")
                .contains("could not be parsed")
        );
    }

    #[test]
    fn rejects_a_provider_without_an_inference_method() {
        assert!(
            rejection_for(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n"
            )
            .contains("no #[infer_from_request] method")
        );
    }

    #[test]
    fn rejects_a_provider_with_more_than_one_inference_method() {
        assert!(
            rejection_for(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n\nimpl Bad {\n    #[infer_from_request]\n    fn first(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n\n    #[infer_from_request]\n    fn second(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n"
            )
            .contains("more than one #[infer_from_request] method")
        );
    }

    #[test]
    fn orders_the_providers_by_the_struct_that_declares_them() {
        let registries = registries_for(
            "struct Admin;\n\n#[singleton]\n#[infers_authenticated_user(user_model = Admin)]\nstruct SecondProvider;\n\nimpl SecondProvider {\n    #[infer_from_request]\n    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<Admin>> {}\n}\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct FirstProvider;\n\nimpl FirstProvider {\n    #[infer_from_request]\n    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
        );
        let providers = registries.providers();

        assert_eq!(
            providers
                .iter()
                .map(|provider| provider.application.concrete.to_string())
                .collect::<Vec<String>>(),
            vec![
                "crate::FirstProvider".to_string(),
                "crate::SecondProvider".to_string()
            ]
        );
    }

    #[test]
    fn rejects_two_providers_for_the_same_user_model() {
        assert!(
            rejection_for(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct First;\n\nimpl First {\n    #[infer_from_request]\n    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct Second;\n\nimpl Second {\n    #[infer_from_request]\n    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n"
            )
            .contains("has more than one authenticated user provider")
        );
    }

    #[test]
    fn rejects_the_next_handler_in_an_inference_method() {
        assert!(
            rejection_for(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n\nimpl Bad {\n    #[infer_from_request]\n    fn infer(&self, next: Next) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n"
            )
            .contains("only available inside an HTTP middleware")
        );
    }

    #[test]
    fn rejects_a_route_parameter_in_an_inference_method() {
        assert!(
            rejection_for(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n\nimpl Bad {\n    #[infer_from_request]\n    fn infer(&self, #[route_parameter(from = \"id\")] id: String) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n"
            )
            .contains("has no route path to bind from")
        );
    }

    #[test]
    fn rejects_an_authenticated_user_in_an_inference_method() {
        assert!(
            rejection_for(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n\nimpl Bad {\n    #[infer_from_request]\n    fn infer(&self, #[authenticated_user] user: User) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n"
            )
            .contains("only available in an HTTP responder")
        );
    }

    #[test]
    fn rejects_the_request_cancellation_token_in_an_inference_method() {
        assert!(
            rejection_for(
                "use tokio_util::sync::CancellationToken;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n\nimpl Bad {\n    #[infer_from_request]\n    fn infer(&self, cancellation: &CancellationToken) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n"
            )
            .contains("without awaiting cancellable work")
        );
    }

    #[test]
    fn rejects_an_unmarked_parameter_in_an_inference_method() {
        assert!(
            rejection_for(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n\nimpl Bad {\n    #[infer_from_request]\n    fn infer(&self, flag: bool) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n"
            )
            .contains("must be the current request, a form request")
        );
    }

    fn responder_binding(
        identifier: &str,
        declaration: &str,
    ) -> Result<Vec<BoundParameter>, RequestBindingError> {
        let index = index_for(&provider_source(&format!(
            "{SESSION_PROVIDER}{declaration}"
        )));
        let registries = BindingRegistries::collect(&index, ViewsAvailability::Available)
            .expect("the binding registries are collected");
        let item = index
            .items()
            .iter()
            .find(|item| item.identifier() == identifier)
            .expect("the responder is indexed");
        let route_path = RoutePath::parse("/{x}");
        let method = process_method(item).expect("the responder has a #[process] method");

        classify_parameters(
            &index,
            item,
            method,
            &BindingContext::Responder {
                route_path: &route_path,
                server: "public",
                subject: "responder 'Page'",
            },
            &registries,
        )
    }

    fn responder_rejection(declaration: &str) -> String {
        responder_binding("Page", declaration)
            .err()
            .expect("the responder is rejected")
            .to_string()
    }

    fn authenticated_user_requirements(declaration: &str) -> Vec<AuthenticatedUserRequirement> {
        responder_binding("Page", declaration)
            .expect("the responder binds")
            .iter()
            .filter_map(|parameter| match &parameter.binding {
                RequestBinding::AuthenticatedUser { requirement, .. } => Some(*requirement),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn binds_a_required_authenticated_user() {
        assert_eq!(
            authenticated_user_requirements(
                "struct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[authenticated_user] user: User) -> anyhow::Result<Response> {}\n}\n"
            ),
            vec![AuthenticatedUserRequirement::Required]
        );
    }

    #[test]
    fn binds_an_optional_authenticated_user() {
        assert_eq!(
            authenticated_user_requirements(
                "struct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[authenticated_user] user: Option<User>) -> anyhow::Result<Response> {}\n}\n"
            ),
            vec![AuthenticatedUserRequirement::Optional]
        );
    }

    #[test]
    fn binds_an_authenticated_user_alongside_the_current_request() {
        assert_eq!(
            authenticated_user_requirements(
                "struct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, request: &Request, #[authenticated_user] user: User) -> anyhow::Result<Response> {}\n}\n"
            ),
            vec![AuthenticatedUserRequirement::Required]
        );
    }

    #[test]
    fn rejects_an_authenticated_user_taken_by_reference() {
        assert!(
            responder_rejection(
                "struct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[authenticated_user] user: &User) -> anyhow::Result<Response> {}\n}\n"
            )
            .contains("must be taken by value")
        );
    }

    #[test]
    fn rejects_an_authenticated_user_model_that_matches_no_struct() {
        assert!(
            responder_rejection(
                "struct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[authenticated_user] user: (u8, u8)) -> anyhow::Result<Response> {}\n}\n"
            )
            .contains("which matches no struct")
        );
    }

    #[test]
    fn rejects_an_authenticated_user_model_without_a_provider() {
        assert!(
            responder_rejection(
                "struct Admin;\n\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[authenticated_user] admin: Admin) -> anyhow::Result<Response> {}\n}\n"
            )
            .contains("which no #[infers_authenticated_user] provides")
        );
    }

    #[test]
    fn rejects_an_authenticated_user_that_also_carries_a_route_parameter() {
        assert!(
            responder_rejection(
                "struct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[authenticated_user] #[route_parameter(from = \"x\")] user: User) -> anyhow::Result<Response> {}\n}\n"
            )
            .contains("an argument may use at most one")
        );
    }

    #[test]
    fn rejects_the_same_authenticated_user_requested_twice() {
        assert!(
            responder_rejection(
                "struct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[authenticated_user] first: User, #[authenticated_user] second: Option<User>) -> anyhow::Result<Response> {}\n}\n"
            )
            .contains("more than once")
        );
    }

    #[test]
    fn rejects_the_same_route_parameter_bound_twice() {
        assert!(
            responder_rejection(
                "struct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"x\")] first: String, #[route_parameter(from = \"x\")] second: String) -> anyhow::Result<Response> {}\n}\n"
            )
            .contains("requests route parameter 'x' more than once")
        );
    }

    #[test]
    fn rejects_a_provider_that_renders_views_this_crate_does_not_generate() {
        let rejection = BindingRegistries::collect(
            &index_for(&provider_source(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n\nimpl Bad {\n    #[infer_from_request]\n    fn infer(&self, views: &crate::margaret::views::Views) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
            )),
            ViewsAvailability::Unavailable,
        )
        .err()
        .expect("the binding registries are rejected")
        .to_string();

        assert!(rejection.contains("requests the views, but this crate generates none"));
    }

    const CONSOLE_ARGUMENT_PROVIDER: &str = "\
#[singleton]
#[infers_authenticated_user(user_model = User)]
struct SessionUserProvider;

impl SessionUserProvider {
    #[constructor]
    fn create(#[console_argument(from = \"realm\")] realm: String) -> anyhow::Result<Self> {}

    #[infer_from_request]
    fn infer(&self) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}
}
";

    #[test]
    fn collects_the_console_arguments_a_binding_pulls_in() {
        let index = index_for(&provider_source(CONSOLE_ARGUMENT_PROVIDER));
        let registry = scan(&index).expect("the console arguments are scanned");
        let bindings = render_container(&index, &registry, &[])
            .expect("the container renders")
            .bindings;
        let registries = BindingRegistries::collect(&index, ViewsAvailability::Available)
            .expect("the binding registries are collected");
        let providers = registries.providers();
        let application = providers
            .first()
            .map(|provider| provider.application.clone())
            .expect("the provider is registered");
        let names: Vec<String> = binding_console_arguments(
            &RequestBinding::AuthenticatedUser {
                application,
                requirement: AuthenticatedUserRequirement::Required,
            },
            &bindings,
        )
        .expect("the provider dependency has planned console arguments")
        .iter()
        .map(|argument| argument.name().to_string())
        .collect();

        assert_eq!(names, vec!["realm".to_string()]);
        assert!(
            binding_console_arguments(&RequestBinding::CurrentRequest, &bindings)
                .expect("the current request has no container-backed arguments")
                .is_empty()
        );
    }

    #[test]
    fn rejects_container_backed_bindings_absent_from_the_container_plan() {
        let bindings = empty_bindings();
        let authenticated_user = RequestBinding::AuthenticatedUser {
            application: AuthenticatedUserApplication {
                concrete: missing_path(),
                field: "missing".to_string(),
                injects_routes: false,
                injects_views: false,
                model: missing_path(),
                wrapper: format_ident!("Missing"),
            },
            requirement: AuthenticatedUserRequirement::Required,
        };
        let bound = RequestBinding::Bound {
            binder_field: "missing".to_string(),
            binder_provider: missing_path(),
            path_key: "id".to_string(),
        };
        let injectable = RequestBinding::Injectable {
            dependency: InjectedDependency {
                concrete: missing_path(),
                field: "missing".to_string(),
            },
        };

        for binding in [authenticated_user, bound, injectable] {
            let error = binding_console_arguments(&binding, &bindings)
                .expect_err("the binding must belong to the same container plan");

            assert!(error.to_string().contains("crate::Missing"));
        }
    }

    #[test]
    fn renders_a_wrapper_that_infers_through_the_declared_method() {
        let registries = registries_for(SESSION_PROVIDER);
        let source: String = render_authenticated_user_wrappers(&registries.providers())
            .into_iter()
            .map(|module| module.to_source())
            .collect::<String>()
            .split_whitespace()
            .collect();

        assert!(source.contains(
            "pubstructSessionUserProvider{pubinner:std::sync::Arc<crate::SessionUserProvider>,}"
        ));
        assert!(source.contains("typeUser=crate::User;"));
        assert!(source.contains("self.inner.infer(request).await"));
    }

    #[test]
    fn renders_a_wrapper_that_carries_the_routes_and_views_it_infers_from() {
        let registries = registries_for(
            "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct SessionUserProvider;\n\nimpl SessionUserProvider {\n    #[infer_from_request]\n    fn infer(&self, routes: &crate::margaret::routes::Routes, views: &crate::margaret::views::Views) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
        );
        let source: String = render_authenticated_user_wrappers(&registries.providers())
            .into_iter()
            .map(|module| module.to_source())
            .collect::<String>()
            .split_whitespace()
            .collect();

        assert!(source.contains("pubroutes:std::sync::Arc<super::super::routes::Routes>,"));
        assert!(source.contains("pubviews:std::sync::Arc<super::super::views::Views>,"));
        assert!(source.contains("self.inner.infer(&self.routes,&self.views).await"));
    }

    #[test]
    fn renders_a_wrapper_that_interrupts_a_failed_form_request() {
        let registries = registries_for(
            "struct Cookie;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct SessionUserProvider;\n\nimpl SessionUserProvider {\n    #[infer_from_request]\n    fn infer(&self, #[form_request(from = Cookie)] cookie: Cookie) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
        );
        let source: String = render_authenticated_user_wrappers(&registries.providers())
            .into_iter()
            .map(|module| module.to_source())
            .collect::<String>()
            .split_whitespace()
            .collect();

        assert!(source.contains("return::std::result::Result::Ok("));
        assert!(source.contains(
            "margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(response.into(),)"
        ));
    }
}
