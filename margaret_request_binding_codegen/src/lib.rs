pub mod authenticated_user_application;
pub mod authenticated_user_challenge;
pub mod authenticated_user_provider;
pub mod authenticated_user_providers;
pub mod authenticated_user_requirement;
mod bearer_token_verifier_field;
pub mod binding_context;
pub mod binding_reads_request;
pub mod binding_registries;
pub mod binding_roots;
pub mod binding_serve_inputs;
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
pub mod injects_peer_spiffe_id;
pub mod injects_routes;
pub mod injects_views;
pub mod render_authenticated_user_wrapper_construction;
pub mod render_authenticated_user_wrappers;
pub mod render_bound_request_extractions;
pub mod render_request_extraction;
pub mod request_binding;
pub mod request_binding_error;
pub mod request_injectable;
pub mod request_input_source;
mod route_parameter_arguments;
pub mod route_parameter_binder;
pub mod route_parameter_resolution;
pub mod route_parameter_resolutions;
pub mod views_availability;

#[cfg(test)]
mod tests {
    use quote::format_ident;
    use syn::Path;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::tag::Tag;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::framework_construction::FrameworkConstruction;
    use margaret_container::framework_dependency::FrameworkDependency;
    use margaret_container::framework_enablement::FrameworkEnablement;
    use margaret_container::framework_injection_role::FrameworkInjectionRole;
    use margaret_container::framework_provider::FrameworkProvider;
    use margaret_container::injected_dependency::InjectedDependency;
    use margaret_container::render_container::render_container;
    use margaret_injection_codegen::process_method::process_method;
    use margaret_route_parameter_codegen::route_path::RoutePath;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_tag_codegen::tag_pool::TagPool;

    use crate::authenticated_user_application::AuthenticatedUserApplication;
    use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
    use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
    use crate::binding_context::BindingContext;
    use crate::binding_registries::BindingRegistries;
    use crate::binding_serve_inputs::binding_serve_inputs;
    use crate::bound_parameter::BoundParameter;
    use crate::classify_parameters::classify_parameters;
    use crate::extraction_context::ExtractionContext;
    use crate::render_authenticated_user_wrappers::render_authenticated_user_wrappers;
    use crate::render_request_extraction::render_request_extraction;
    use crate::request_binding::RequestBinding;
    use crate::request_binding_error::RequestBindingError;
    use crate::views_availability::ViewsAvailability;

    const PRELUDE: &str = "\
use margaret::framework::http::next::Next;
use margaret::framework::http::request::Request;
use margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome;

struct User;
";

    fn provider_source(declaration: &str) -> String {
        format!("{PRELUDE}{declaration}")
    }

    fn empty_bindings() -> ContainerBindings {
        let index = IndexedSource::new("").index;
        let registry = scan(&index).expect("the empty console argument registry is scanned");

        render_container(&index, &registry, &[])
            .expect("the empty container is rendered")
            .bindings
    }

    fn missing_path() -> CanonicalPath {
        CanonicalPath::new(vec!["crate".to_string(), "Missing".to_string()])
    }

    fn token_issuer_client(tag: &str, provided: &[&str]) -> FrameworkProvider {
        let path: Path = syn::parse_str(tag).expect("the tag path parses");

        FrameworkProvider {
            construction: FrameworkConstruction::Unit,
            enablement: FrameworkEnablement::Always,
            injection: FrameworkInjectionRole::TokenIssuerClient(
                Tag::from_path(&path).expect("the tag is a plain name"),
            ),
            provided: CanonicalPath::new(provided.iter().map(ToString::to_string).collect()),
        }
    }

    fn oidc_client(issuer: &str) -> FrameworkProvider {
        token_issuer_client(issuer, &["crate", "margaret", "oidc", issuer, "OidcClient"])
    }

    fn token_issuer_client_bindings() -> ContainerBindings {
        let index = IndexedSource::new("").index;

        render_container(
            &index,
            &scan(&index).expect("the serve inputs are scanned"),
            &[
                oidc_client("partner"),
                oidc_client("upstream"),
                token_issuer_client(
                    "auth",
                    &["crate", "margaret", "jwks", "auth_endpoint", "JwksClient"],
                ),
            ],
        )
        .expect("the container renders")
        .bindings
    }

    fn collect_registries(
        index: &AttributeIndex,
        views: ViewsAvailability,
    ) -> Result<BindingRegistries, RequestBindingError> {
        BindingRegistries::collect(
            index,
            views,
            &TagPool::collect(index).expect("the tags are collected"),
            &token_issuer_client_bindings(),
        )
    }

    fn registries_with(declaration: &str, views: ViewsAvailability) -> BindingRegistries {
        collect_registries(
            &IndexedSource::new(&provider_source(declaration)).index,
            views,
        )
        .expect("the binding registries are collected")
    }

    fn registries_for(declaration: &str) -> BindingRegistries {
        registries_with(declaration, ViewsAvailability::Available)
    }

    fn rejection_for(declaration: &str) -> String {
        collect_registries(
            &IndexedSource::new(&provider_source(declaration)).index,
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
    fn flags_only_a_provider_that_reads_the_peer_spiffe_id() {
        let peer_reading = registries_for(
            "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct SessionUserProvider;\n\nimpl SessionUserProvider {\n    #[infer_from_request]\n    fn infer(&self, peer: &spiffe::spiffe_id::SpiffeId) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
        );
        let request_reading = registries_for(SESSION_PROVIDER);

        assert!(
            peer_reading.providers()[0]
                .application
                .injects_peer_spiffe_id
        );
        assert!(
            !request_reading.providers()[0]
                .application
                .injects_peer_spiffe_id
        );
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
        let indexed = IndexedSource::new(&provider_source(&format!(
            "{SESSION_PROVIDER}{declaration}"
        )));
        let index = &indexed.index;
        let registries = collect_registries(index, ViewsAvailability::Available)
            .expect("the binding registries are collected");
        let item = indexed.item(identifier);
        let route_path = RoutePath::parse("/{x}");
        let method = process_method(item).expect("the responder has a #[process] method");

        classify_parameters(
            index,
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
    fn rejects_a_user_defined_option_as_an_optional_authenticated_user() {
        assert!(
            responder_rejection(
                "struct Option<Inner>(Inner);\n\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[authenticated_user] user: Option<User>) -> anyhow::Result<Response> {}\n}\n"
            )
            .contains("requests the authenticated user 'crate::Option'")
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
    fn rejects_a_form_request_source_named_after_a_local_struct() {
        assert!(
            responder_rejection(
                "struct Cookie;\n\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[form_request(from = Cookie)] cookie: Cookie) -> anyhow::Result<Response> {}\n}\n"
            )
            .contains("unknown request input source 'Cookie'")
        );
    }

    #[test]
    fn rejects_a_form_request_source_from_a_foreign_enum() {
        assert!(
            responder_rejection(
                "use crate::inputs::RequestInput;\n\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[form_request(from = RequestInput::Query)] filters: Filters) -> anyhow::Result<Response> {}\n}\n"
            )
            .contains("unknown request input source 'RequestInput :: Query'")
        );
    }

    #[test]
    fn binds_a_form_request_source_imported_from_the_framework() {
        assert!(
            responder_binding(
                "Page",
                "use margaret::framework::http_validation::request_input::RequestInput;\n\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[form_request(from = RequestInput::Query)] filters: Filters) -> anyhow::Result<Response> {}\n}\n"
            )
            .is_ok()
        );
    }

    #[test]
    fn binds_a_form_request_source_imported_as_a_variant() {
        assert!(
            responder_binding(
                "Page",
                "use margaret::framework::http_validation::request_input::RequestInput::Cookie;\n\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[form_request(from = Cookie)] cookie: Filters) -> anyhow::Result<Response> {}\n}\n"
            )
            .is_ok()
        );
    }

    #[test]
    fn binds_a_form_request_source_through_a_reexported_module() {
        assert!(
            responder_binding(
                "Page",
                "mod prelude {\n    pub use margaret::framework::http_validation::request_input;\n}\n\nuse crate::prelude::request_input;\n\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[form_request(from = request_input::RequestInput::Form)] form: Filters) -> anyhow::Result<Response> {}\n}\n"
            )
            .is_ok()
        );
    }

    #[test]
    fn resolves_a_user_model_imported_through_a_reexport() {
        let registries = registries_for(
            "mod accounts {\n    mod member {\n        pub struct Member;\n    }\n\n    pub use member::Member;\n}\n\nuse crate::accounts::Member;\n\n#[singleton]\n#[infers_authenticated_user(user_model = Member)]\nstruct MemberProvider;\n\nimpl MemberProvider {\n    #[infer_from_request]\n    fn infer(&self, request: &Request) -> anyhow::Result<AuthenticatedUserOutcome<Member>> {}\n}\n",
        );

        assert_eq!(
            registries.providers()[0].application.model.to_string(),
            "crate::accounts::member::Member"
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
        let rejection = collect_registries(
            &IndexedSource::new(&provider_source(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n\nimpl Bad {\n    #[infer_from_request]\n    fn infer(&self, views: &crate::margaret::views::Views) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
            )).index,
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
        let index = IndexedSource::new(&provider_source(CONSOLE_ARGUMENT_PROVIDER)).index;
        let registry = scan(&index).expect("the console arguments are scanned");
        let bindings = render_container(&index, &registry, &[])
            .expect("the container renders")
            .bindings;
        let registries = collect_registries(&index, ViewsAvailability::Available)
            .expect("the binding registries are collected");
        let providers = registries.providers();
        let application = providers
            .first()
            .map(|provider| provider.application.clone())
            .expect("the provider is registered");
        let names: Vec<String> = binding_serve_inputs(
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
            binding_serve_inputs(&RequestBinding::CurrentRequest, &bindings)
                .expect("the current request has no container-backed arguments")
                .is_empty()
        );
    }

    #[test]
    fn rejects_container_backed_bindings_absent_from_the_container_plan() {
        let bindings = empty_bindings();
        let authenticated_user = RequestBinding::AuthenticatedUser {
            application: AuthenticatedUserApplication {
                challenge: AuthenticatedUserChallenge::Unchallenged,
                concrete: missing_path(),
                field: "missing".to_string(),
                injects_peer_spiffe_id: false,
                injects_routes: false,
                injects_views: false,
                model: missing_path(),
                wrapper: format_ident!("Missing"),
            },
            requirement: AuthenticatedUserRequirement::Required,
        };
        let bound = RequestBinding::BoundRouteParameter {
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
            let error = binding_serve_inputs(&binding, &bindings)
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
        assert!(source.contains("self.inner.infer(request)"));
    }

    #[test]
    fn awaits_a_provider_that_declares_an_asynchronous_inference_method() {
        let registries = registries_for(
            "#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct SessionUserProvider;\n\nimpl SessionUserProvider {\n    #[infer_from_request]\n    async fn infer(&self, request: &Request) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
        );
        let source: String = render_authenticated_user_wrappers(&registries.providers())
            .into_iter()
            .map(|module| module.to_source())
            .collect::<String>()
            .split_whitespace()
            .collect();

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
        assert!(source.contains("self.inner.infer(&self.routes,&self.views)"));
    }

    #[test]
    fn renders_a_wrapper_that_interrupts_a_failed_form_request() {
        let registries = registries_for(
            "use margaret::framework::http_validation::request_input::RequestInput;\n\nstruct Cookie;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct SessionUserProvider;\n\nimpl SessionUserProvider {\n    #[infer_from_request]\n    fn infer(&self, #[form_request(from = RequestInput::Cookie)] cookie: Cookie) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
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

    const PARTNER_ISSUER: &str = "\
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;

#[singleton]
#[trusts_oidc_issuer(partner)]
struct PartnerIssuer;

struct Claims;
";

    fn bearer_provider(parameters: &str) -> String {
        format!(
            "{PARTNER_ISSUER}\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct RunnerProvider;\n\nimpl RunnerProvider {{\n    #[infer_from_request]\n    fn infer(&self, {parameters}) -> anyhow::Result<AuthenticatedUserOutcome<User>> {{}}\n}}\n"
        )
    }

    fn bearer_rejection(parameters: &str) -> String {
        rejection_for(&bearer_provider(parameters))
    }

    fn wrapper_source(registries: &BindingRegistries) -> String {
        render_authenticated_user_wrappers(&registries.providers())
            .into_iter()
            .map(|module| module.to_source())
            .collect::<String>()
            .split_whitespace()
            .collect()
    }

    fn verifies_with(registries: &BindingRegistries, client: &str) -> bool {
        matches!(
            &registries.providers()[0].application.challenge,
            AuthenticatedUserChallenge::Bearer { issuer_client }
                if issuer_client.concrete.to_string() == client
        )
    }

    const PARTNER_TOKEN: &str =
        "#[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Claims>>";

    #[test]
    fn binds_a_bearer_token_to_the_client_of_its_oidc_issuer() {
        assert!(verifies_with(
            &registries_for(&bearer_provider(PARTNER_TOKEN)),
            "crate::margaret::oidc::partner::OidcClient"
        ));
    }

    #[test]
    fn binds_a_bearer_token_to_the_client_of_its_jwks_endpoint() {
        assert!(verifies_with(
            &registries_for(&format!(
                "#[singleton]\n#[provides_jwks_endpoint(auth)]\nstruct AuthEndpoint;\n\n{}",
                bearer_provider(
                    "#[bearer_token(issuer = auth)] token: Option<VerifiedJwt<Claims>>"
                )
            )),
            "crate::margaret::jwks::auth_endpoint::JwksClient"
        ));
    }

    #[test]
    fn renders_no_bearer_token_verifier_for_a_provider_without_a_bearer_token() {
        assert!(
            !wrapper_source(&registries_for(SESSION_PROVIDER)).contains("bearer_token_verifier")
        );
    }

    #[test]
    fn renders_a_wrapper_that_admits_the_bearer_token_with_the_verifier_of_its_issuer() {
        let source = wrapper_source(&registries_for(&bearer_provider(&format!(
            "request: &Request, {PARTNER_TOKEN}"
        ))));

        assert!(source.contains(
            "pubbearer_token_verifier:std::sync::Arc<margaret::framework::bearer_token_verification::bearer_token_verifier::BearerTokenVerifier>,"
        ));
        assert!(source.contains(
            "lettoken=matchmargaret::framework::bearer_token_verification::admit_bearer_token::admit_bearer_token::<crate::Claims>(&self.bearer_token_verifier,request.inputs.server.authorization(),).map_err(margaret::framework::anyhow::Error::from){::std::result::Result::Ok(margaret::framework::bearer_token_verification::bearer_token_admission::BearerTokenAdmission::Anonymous)=>::std::option::Option::None,::std::result::Result::Ok(margaret::framework::bearer_token_verification::bearer_token_admission::BearerTokenAdmission::Presented(token))=>::std::option::Option::Some(token),::std::result::Result::Ok(margaret::framework::bearer_token_verification::bearer_token_admission::BearerTokenAdmission::Refused(response))=>return::std::result::Result::Ok(margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(response,),),::std::result::Result::Err(error)=>return::std::result::Result::Err(error),};"
        ));
        assert!(source.contains("self.inner.infer(request,token)"));
    }

    #[test]
    fn resolves_the_bearer_token_type_and_claims_through_use_statements() {
        let source = wrapper_source(&registries_for(
            "use margaret::framework::jwt_verification::verified_jwt;\n\nmod ci {\n    mod claims {\n        pub struct Claims;\n    }\n\n    pub use claims::Claims;\n}\n\nuse crate::ci::Claims;\n\n#[singleton]\n#[trusts_oidc_issuer(partner)]\nstruct PartnerIssuer;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct RunnerProvider;\n\nimpl RunnerProvider {\n    #[infer_from_request]\n    fn infer(&self, #[bearer_token(issuer = partner)] token: Option<verified_jwt::VerifiedJwt<Claims>>) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
        ));

        assert!(source.contains("admit_bearer_token::<crate::ci::claims::Claims>("));
    }

    #[test]
    fn rejects_a_bearer_token_without_an_issuer() {
        assert!(
            bearer_rejection("#[bearer_token] token: Option<VerifiedJwt<Claims>>")
                .ends_with("must be `issuer = <tag>`")
        );
    }

    #[test]
    fn propagates_malformed_bearer_token_arguments() {
        assert!(
            bearer_rejection("#[bearer_token(= 5)] token: Option<VerifiedJwt<Claims>>")
                .contains("could not be parsed")
        );
    }

    #[test]
    fn rejects_a_bearer_token_of_an_undeclared_issuer() {
        assert!(
            bearer_rejection(
                "#[bearer_token(issuer = undeclared)] token: Option<VerifiedJwt<Claims>>"
            )
            .ends_with("references the tag 'undeclared', which no token issuer declares")
        );
    }

    #[test]
    fn rejects_a_bearer_token_of_a_tag_that_names_no_token_issuer() {
        assert!(
            rejection_for(&format!(
                "#[singleton]\n#[handles_middleware_attribute(attribute = logged)]\nstruct RequestLog;\n\n{}",
                bearer_provider("#[bearer_token(issuer = logged)] token: Option<VerifiedJwt<Claims>>")
            ))
            .ends_with("references the tag 'logged', which is a middleware handler, not a token issuer")
        );
    }

    #[test]
    fn rejects_a_bearer_token_declared_as_another_type() {
        assert!(
            bearer_rejection("#[bearer_token(issuer = partner)] token: String")
                .contains("carries #[bearer_token] on 'String'")
        );
    }

    #[test]
    fn rejects_a_bearer_token_that_is_not_optional() {
        assert!(
            bearer_rejection("#[bearer_token(issuer = partner)] token: VerifiedJwt<Claims>")
                .contains("carries #[bearer_token] on 'VerifiedJwt < Claims >'")
        );
    }

    #[test]
    fn rejects_a_bearer_token_taken_by_reference() {
        assert!(
            bearer_rejection(
                "#[bearer_token(issuer = partner)] token: Option<&VerifiedJwt<Claims>>"
            )
            .contains("taken by value")
        );
    }

    #[test]
    fn rejects_bearer_token_claims_with_generic_arguments() {
        assert!(
            bearer_rejection(
                "#[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Vec<Claims>>>"
            )
            .ends_with(
                "into the claims 'Vec < Claims >', which is not a named type without generic arguments"
            )
        );
    }

    #[test]
    fn rejects_bearer_token_claims_that_are_not_a_named_type() {
        assert!(
            bearer_rejection(
                "#[bearer_token(issuer = partner)] token: Option<VerifiedJwt<(Claims, Claims)>>"
            )
            .ends_with(
                "into the claims '(Claims , Claims)', which is not a named type without generic arguments"
            )
        );
    }

    #[test]
    fn rejects_bearer_token_claims_that_match_no_type() {
        assert!(
            bearer_rejection("#[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Ghost>>")
                .ends_with("into the claims 'Ghost', which matches no type in scope")
        );
    }

    #[test]
    fn rejects_a_provider_that_reads_the_bearer_token_twice() {
        assert!(
            rejection_for(&format!(
                "#[singleton]\n#[trusts_oidc_issuer(upstream)]\nstruct UpstreamIssuer;\n\n{}",
                bearer_provider(&format!(
                    "{PARTNER_TOKEN}, #[bearer_token(issuer = upstream)] upstream: Option<VerifiedJwt<Claims>>"
                ))
            ))
            .ends_with(
                "reads the bearer token more than once; a request presents exactly one bearer token"
            )
        );
    }

    #[test]
    fn rejects_a_bearer_token_that_also_carries_a_form_request() {
        assert!(
            bearer_rejection(
                "#[bearer_token(issuer = partner)] #[form_request(from = RequestInput::Query)] token: Option<VerifiedJwt<Claims>>"
            )
            .contains("carries #[bearer_token] together with")
        );
    }

    #[test]
    fn rejects_a_bearer_token_on_the_peer_spiffe_id() {
        assert!(
            bearer_rejection(
                "#[bearer_token(issuer = partner)] peer: &spiffe::spiffe_id::SpiffeId"
            )
            .contains("is the peer SPIFFE id and must not also carry")
        );
    }

    #[test]
    fn rejects_a_bearer_token_outside_an_inference_method() {
        assert!(
            responder_rejection(&format!(
                "{PARTNER_ISSUER}\nstruct Page;\n\nimpl Page {{\n    #[process]\n    fn respond(&self, {PARTNER_TOKEN}) -> anyhow::Result<Response> {{}}\n}}\n"
            ))
            .contains("which is only available in an #[infer_from_request] method")
        );
    }

    #[test]
    fn rejects_a_bearer_token_whose_client_the_container_does_not_plan() {
        let index = IndexedSource::new(&provider_source(&bearer_provider(PARTNER_TOKEN))).index;
        let rejection = BindingRegistries::collect(
            &index,
            ViewsAvailability::Available,
            &TagPool::collect(&index).expect("the tags are collected"),
            &empty_bindings(),
        )
        .err()
        .expect("the binding registries are rejected")
        .to_string();

        assert!(rejection.ends_with(
            "verifies the bearer token of the token issuer 'partner', whose client the container does not plan"
        ));
    }

    #[test]
    fn challenges_a_required_user_of_a_bearer_provider_for_bearer_credentials() {
        let indexed = IndexedSource::new(&provider_source(&format!(
            "{}\nstruct Page;\n\nimpl Page {{\n    #[process]\n    fn respond(&self, #[authenticated_user] runner: User) -> anyhow::Result<Response> {{}}\n}}\n",
            bearer_provider(PARTNER_TOKEN)
        )));
        let registries = collect_registries(&indexed.index, ViewsAvailability::Available)
            .expect("the binding registries are collected");
        let item = indexed.item("Page");
        let route_path = RoutePath::parse("/");
        let parameters = classify_parameters(
            &indexed.index,
            item,
            process_method(item).expect("the responder has a #[process] method"),
            &BindingContext::Responder {
                route_path: &route_path,
                server: "public",
                subject: "responder 'Page'",
            },
            &registries,
        )
        .expect("the responder binds");
        let extraction: String = parameters
            .iter()
            .map(|parameter| {
                render_request_extraction(
                    &parameter.binding,
                    &parameter.holder,
                    &ExtractionContext {
                        continuation_return: &quote::quote! { return response },
                        error_return: &quote::quote! { return error },
                        provider_access: &quote::quote! { provider },
                        request_local: &format_ident!("request"),
                        response_return: &quote::quote! { return response },
                    },
                )
                .to_string()
            })
            .collect::<String>()
            .split_whitespace()
            .collect();

        assert!(extraction.contains(
            "margaret::framework::identity::require_bearer_authenticated_user::require_bearer_authenticated_user(outcome)"
        ));
    }

    #[test]
    fn refuses_to_inject_a_token_issuer_client_into_a_handshake() {
        let indexed = IndexedSource::new(
            "struct Room;\n\nimpl Room {\n    #[process]\n    fn build(client: std::sync::Arc<crate::margaret::oidc::partner::OidcClient>) -> anyhow::Result<Self> {}\n}\n",
        );
        let item = indexed.item("Room");
        let route_path = RoutePath::parse("/room");
        let bindings = token_issuer_client_bindings();
        let rejection = classify_parameters(
            &indexed.index,
            item,
            process_method(item).expect("the builder has a #[process] method"),
            &BindingContext::Handshake {
                container_bindings: &bindings,
                route_path: &route_path,
                server: "public",
                subject: "session 'Room'",
            },
            &collect_registries(&indexed.index, ViewsAvailability::Available)
                .expect("the binding registries are collected"),
        )
        .err()
        .expect("the handshake is rejected")
        .to_string();

        assert_eq!(
            rejection,
            "parameter '0' of session 'Room' injects a provider by its path, which only the framework may inject"
        );
    }

    #[test]
    fn rejects_a_site_that_infers_two_users_from_the_bearer_token() {
        assert_eq!(
            responder_rejection(&format!(
                "{}\nstruct Runner;\n\nstruct Admin;\n\n#[singleton]\n#[infers_authenticated_user(user_model = Admin)]\nstruct AdminProvider;\n\nimpl AdminProvider {{\n    #[infer_from_request]\n    fn infer(&self, #[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Claims>>) -> anyhow::Result<AuthenticatedUserOutcome<Admin>> {{}}\n}}\n\nstruct Page;\n\nimpl Page {{\n    #[process]\n    fn respond(&self, #[authenticated_user] runner: Runner, #[authenticated_user] admin: Admin) -> anyhow::Result<Response> {{}}\n}}\n",
                bearer_provider(PARTNER_TOKEN).replace("User", "Runner")
            )),
            "responder 'Page' infers more than one authenticated user from the bearer token; a request presents one bearer credential, so exactly one #[infers_authenticated_user] provider verifies it"
        );
    }

    #[test]
    fn collects_the_console_arguments_of_the_issuer_a_bearer_provider_verifies() {
        let index = IndexedSource::new(&provider_source(&format!(
            "use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;\nuse margaret::framework::token_trust::declares_token_trust::DeclaresTokenTrust;\n\nstruct Claims;\n\n#[singleton]\n#[trusts_oidc_issuer(partner)]\nstruct PartnerIssuer;\n\nimpl DeclaresTokenTrust for PartnerIssuer {{}}\n\nimpl PartnerIssuer {{\n    #[constructor]\n    fn create(#[console_argument(from = \"audience\")] audience: String) -> anyhow::Result<Self> {{}}\n}}\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct RunnerProvider;\n\nimpl RunnerProvider {{\n    #[infer_from_request]\n    fn infer(&self, {PARTNER_TOKEN}) -> anyhow::Result<AuthenticatedUserOutcome<User>> {{}}\n}}\n"
        )))
        .index;
        let tags = TagPool::collect(&index).expect("the tags are collected");
        let bindings = render_container(
            &index,
            &scan(&index).expect("the console arguments are scanned"),
            &[FrameworkProvider {
                construction: FrameworkConstruction::Constructor {
                    dependencies: vec![FrameworkDependency::SingletonView(CanonicalPath::new(
                        vec!["crate".to_string(), "PartnerIssuer".to_string()],
                    ))],
                    is_async: false,
                    method: "create".to_string(),
                },
                ..oidc_client("partner")
            }],
        )
        .expect("the container renders")
        .bindings;
        let registries =
            BindingRegistries::collect(&index, ViewsAvailability::Available, &tags, &bindings)
                .expect("the binding registries are collected");
        let names: Vec<String> = binding_serve_inputs(
            &RequestBinding::AuthenticatedUser {
                application: registries.providers()[0].application.clone(),
                requirement: AuthenticatedUserRequirement::Required,
            },
            &bindings,
        )
        .expect("the token issuer client has planned console arguments")
        .iter()
        .map(|argument| argument.name().to_string())
        .collect();

        assert_eq!(names, vec!["audience".to_string()]);
    }

    #[test]
    fn rejects_a_token_issuer_client_absent_from_the_container_plan() {
        let index = IndexedSource::new(&provider_source(SESSION_PROVIDER)).index;
        let bindings = render_container(
            &index,
            &scan(&index).expect("the console arguments are scanned"),
            &[],
        )
        .expect("the container renders")
        .bindings;
        let mut application = registries_for(SESSION_PROVIDER).providers()[0]
            .application
            .clone();

        application.challenge = AuthenticatedUserChallenge::Bearer {
            issuer_client: InjectedDependency {
                concrete: missing_path(),
                field: "missing".to_string(),
            },
        };

        let error = binding_serve_inputs(
            &RequestBinding::AuthenticatedUser {
                application,
                requirement: AuthenticatedUserRequirement::Required,
            },
            &bindings,
        )
        .expect_err("the token issuer client must belong to the same container plan");

        assert!(error.to_string().contains("crate::Missing"));
    }
}
