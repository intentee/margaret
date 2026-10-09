pub mod authenticated_user_application;
pub mod authenticated_user_challenge;
pub mod authenticated_user_provider;
pub mod authenticated_user_providers;
pub mod authenticated_user_requirement;
mod bearer_token_marker;
mod bearer_token_parameter;
pub mod binding_context;
pub mod binding_reads_request;
pub mod binding_registries;
pub mod binding_roots;
mod bound_arguments;
mod bound_credentials;
pub mod bound_parameter;
pub mod captured_provider;
pub mod captured_provider_kind;
pub mod captured_providers;
pub mod classify_parameters;
pub mod content_binding;
pub mod content_extraction_context;
mod exclusive_bindings;
pub mod extraction_context;
mod extraction_phase;
mod form_request_arguments;
pub mod form_request_extraction;
pub mod head_extraction_context;
pub mod head_input_source;
mod inference_channel;
mod infers_authenticated_user_arguments;
pub mod injects_peer_spiffe_id;
pub mod injects_routes;
pub mod injects_views;
mod plain_type_resolution;
mod render_authenticated_user_extraction;
pub mod render_authenticated_user_wrapper_construction;
pub mod render_authenticated_user_wrappers;
mod render_bound_request_extractions;
pub mod render_content_extraction;
pub mod render_head_extractions;
mod render_model_extraction;
pub mod render_request_extraction;
pub mod render_session_user_inference;
pub mod request_binding;
pub mod request_binding_error;
pub mod request_injectable;
pub mod request_input_source;
mod request_inputs;
pub mod responder_content;
pub mod route_database;
pub mod route_model;
pub mod route_model_key;
pub mod route_model_resolution;
mod route_parameter_arguments;
pub mod route_parameter_binder;
pub mod route_parameter_lookup;
pub mod route_parameter_resolution;
pub mod route_parameter_resolutions;
mod session_marker;
pub mod session_user_parameter;
pub mod views_availability;

#[cfg(test)]
mod tests {
    use quote::format_ident;
    use syn::Path;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::name_allocator::NameAllocator;
    use margaret_attributes::tag::Tag;
    use margaret_attributes_tests::indexed_source::IndexedSource;
    use margaret_container::constructor_outcome::ConstructorOutcome;
    use margaret_container::container_bindings::ContainerBindings;
    use margaret_container::framework_construction::FrameworkConstruction;
    use margaret_container::framework_dependency::FrameworkDependency;
    use margaret_container::framework_enablement::FrameworkEnablement;
    use margaret_container::framework_injection_role::FrameworkInjectionRole;
    use margaret_container::framework_provider::FrameworkProvider;
    use margaret_container::render_container::render_container;
    use margaret_database_codegen::database_canonical_path::database_canonical_path;
    use margaret_database_codegen::declared_postgres_database::DeclaredPostgresDatabase;
    use margaret_injection_codegen::process_method::process_method;
    use margaret_route_parameter_codegen::route_path::RoutePath;
    use margaret_serve_input_codegen::scan::scan;
    use margaret_sessions_codegen::sessions_item::SessionsItem;
    use margaret_sessions_codegen::sessions_item_path::sessions_item_path;
    use margaret_tag_codegen::session_source::SessionSource;
    use margaret_tag_codegen_tests::collected_tags::collected_tags;
    use margaret_token_issuance_codegen::declared_token_issuance::DeclaredTokenIssuance;

    use crate::authenticated_user_challenge::AuthenticatedUserChallenge;
    use crate::authenticated_user_requirement::AuthenticatedUserRequirement;
    use crate::binding_context::BindingContext;
    use crate::binding_registries::BindingRegistries;
    use crate::binding_roots::binding_roots;
    use crate::bound_parameter::BoundParameter;
    use crate::captured_providers::CapturedProviders;
    use crate::classify_parameters::classify_parameters;
    use crate::head_extraction_context::HeadExtractionContext;
    use crate::render_authenticated_user_wrappers::render_authenticated_user_wrappers;
    use crate::render_head_extractions::render_head_extractions;
    use crate::render_session_user_inference::render_session_user_inference;
    use crate::request_binding::RequestBinding;
    use crate::request_binding_error::RequestBindingError;
    use crate::route_model::RouteModel;
    use crate::route_model_key::RouteModelKey;
    use crate::route_parameter_lookup::RouteParameterLookup;
    use crate::session_user_parameter::session_user_parameter;
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

        render_container(
            &index,
            &registry,
            &[],
            &DeclaredPostgresDatabase::Absent,
            &DeclaredTokenIssuance::Absent,
        )
        .expect("the empty container is rendered")
        .bindings
    }

    fn trusted_issuer_provider(tag: &str) -> FrameworkProvider {
        let path: Path = syn::parse_str(tag).expect("the tag path parses");

        FrameworkProvider {
            construction: FrameworkConstruction::Unit,
            enablement: FrameworkEnablement::Declared,
            injection: FrameworkInjectionRole::TrustedIssuer(
                Tag::from_path(&path).expect("the tag is a plain name"),
            ),
            provided: CanonicalPath::new(
                ["crate", "margaret", "trusted_issuers", tag, "TrustedIssuer"]
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            ),
        }
    }

    fn oauth_client_provider(tag: &str) -> FrameworkProvider {
        let path: Path = syn::parse_str(tag).expect("the tag path parses");

        FrameworkProvider {
            construction: FrameworkConstruction::Unit,
            enablement: FrameworkEnablement::Declared,
            injection: FrameworkInjectionRole::OAuthClient(
                Tag::from_path(&path).expect("the tag is a plain name"),
            ),
            provided: CanonicalPath::new(
                [
                    "crate",
                    "margaret",
                    "oauth_clients",
                    tag,
                    "AuthorizationServerClient",
                ]
                .iter()
                .map(ToString::to_string)
                .collect(),
            ),
        }
    }

    fn sessions_provider(item: SessionsItem) -> FrameworkProvider {
        FrameworkProvider {
            construction: FrameworkConstruction::Unit,
            enablement: FrameworkEnablement::Declared,
            injection: FrameworkInjectionRole::Unmarked,
            provided: sessions_item_path(item),
        }
    }

    fn trusted_issuer_bindings() -> ContainerBindings {
        let index = IndexedSource::new("").index;

        render_container(
            &index,
            &scan(&index).expect("the serve inputs are scanned"),
            &[
                sessions_provider(SessionsItem::ConsumedSessions),
                sessions_provider(SessionsItem::IssuedSessions),
                trusted_issuer_provider("partner"),
                trusted_issuer_provider("upstream"),
                trusted_issuer_provider("auth"),
                oauth_client_provider("partner_client"),
                oauth_client_provider("second_client"),
            ],
            &DeclaredPostgresDatabase::Absent,
            &DeclaredTokenIssuance::Absent,
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
            &collected_tags(index),
            &trusted_issuer_bindings(),
            &[],
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
        assert_eq!(
            IndexedSource::try_new(&provider_source(
                "#[singleton]\n#[infers_authenticated_user(user_model = User)]\n#[infers_authenticated_user(user_model = User)]\nstruct Bad;\n"
            ))
            .err()
            .expect("the repeated provider attribute is rejected while indexing")
            .to_string(),
            "attribute 'infers_authenticated_user' is repeated on 'crate::Bad' but a single occurrence was expected"
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
        let route_path = RoutePath::parse("/{x}").expect("the route path is routable");
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

    fn database_bindings() -> ContainerBindings {
        let index = IndexedSource::new("").index;

        render_container(
            &index,
            &scan(&index).expect("the serve inputs are scanned"),
            &[FrameworkProvider {
                construction: FrameworkConstruction::Unit,
                enablement: FrameworkEnablement::Declared,
                injection: FrameworkInjectionRole::Unmarked,
                provided: database_canonical_path(),
            }],
            &DeclaredPostgresDatabase::Absent,
            &DeclaredTokenIssuance::Absent,
        )
        .expect("the container renders")
        .bindings
    }

    fn route_model_binding(
        container_bindings: &ContainerBindings,
        primary_key: RouteModelKey,
    ) -> Result<Vec<BoundParameter>, RequestBindingError> {
        let indexed = IndexedSource::new(
            "struct Article;\n\nstruct Page;\n\nimpl Page {\n    #[process]\n    fn respond(&self, #[route_parameter(from = \"x\")] article: Article) -> anyhow::Result<Response> {}\n}\n",
        );
        let index = &indexed.index;
        let registries = BindingRegistries::collect(
            index,
            ViewsAvailability::Available,
            &collected_tags(index),
            container_bindings,
            &[RouteModel {
                loaded: CanonicalPath::new(vec!["crate".to_string(), "Article".to_string()]),
                primary_key,
            }],
        )
        .expect("the binding registries are collected");
        let item = indexed.item("Page");
        let route_path = RoutePath::parse("/{x}").expect("the route path is routable");

        classify_parameters(
            index,
            item,
            process_method(item).expect("the responder has a #[process] method"),
            &BindingContext::Responder {
                route_path: &route_path,
                server: "public",
                subject: "responder 'Page'",
            },
            &registries,
        )
    }

    #[test]
    fn binds_a_single_keyed_model_route_parameter_through_the_database() {
        let bound = route_model_binding(&database_bindings(), RouteModelKey::Single)
            .expect("the route model binds");

        assert!(matches!(
            &bound[0].binding,
            RequestBinding::BoundRouteParameter {
                binder_provider,
                lookup: RouteParameterLookup::PrimaryKey { loaded },
                ..
            } if *binder_provider == database_canonical_path() && loaded.to_string() == "crate::Article"
        ));
    }

    #[test]
    fn rejects_a_composite_keyed_model_route_parameter() {
        assert!(matches!(
            route_model_binding(&database_bindings(), RouteModelKey::Composite),
            Err(RequestBindingError::RouteModelWithCompositePrimaryKey { model, .. })
                if model == "crate::Article"
        ));
    }

    #[test]
    fn rejects_a_model_route_parameter_without_a_database() {
        assert!(matches!(
            route_model_binding(&empty_bindings(), RouteModelKey::Single),
            Err(RequestBindingError::RouteModelWithoutDatabase { model, .. })
                if model == "crate::Article"
        ));
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
        let bindings = render_container(
            &index,
            &registry,
            &[],
            &DeclaredPostgresDatabase::Absent,
            &DeclaredTokenIssuance::Absent,
        )
        .expect("the container renders")
        .bindings;
        let registries = collect_registries(&index, ViewsAvailability::Available)
            .expect("the binding registries are collected");
        let providers = registries.providers();
        let application = providers
            .first()
            .map(|provider| provider.application.clone())
            .expect("the provider is registered");
        let roots: Vec<CanonicalPath> = binding_roots(&RequestBinding::AuthenticatedUser {
            application,
            requirement: AuthenticatedUserRequirement::Required,
        })
        .into_iter()
        .cloned()
        .collect();
        let names: Vec<String> = bindings
            .serve_inputs(&roots)
            .expect("the provider dependency has planned console arguments")
            .iter()
            .map(|argument| argument.input.name().to_string())
            .collect();

        assert_eq!(names, vec!["realm".to_string()]);
        assert!(binding_roots(&RequestBinding::CurrentRequest).is_empty());
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
        assert!(source.contains("letargument_1=request;"));
        assert!(source.contains("self.inner.infer(argument_1)"));
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

        assert!(source.contains("self.inner.infer(argument_1).await"));
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
            "margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(response,)"
        ));
    }

    const PARTNER_ISSUER: &str = "\
use margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile;
use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;

#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]
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

    fn verifies_with(registries: &BindingRegistries, trusted_issuer: &str) -> bool {
        matches!(
            &registries.providers()[0].application.challenge,
            AuthenticatedUserChallenge::Bearer { trusted_issuers }
                if matches!(
                    trusted_issuers.as_slice(),
                    [trusted] if trusted.concrete.to_string() == trusted_issuer
                )
        )
    }

    const PARTNER_TOKEN: &str =
        "#[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Claims, AccessTokenProfile>>";

    const PARTNER_CLIENTS: &str = "\
use margaret::framework::token_introspection::introspected_token::IntrospectedToken;

#[oauth_client(partner_client, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"partner\", issuer = partner)]
struct PartnerClient;

#[oauth_client(second_client, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"second\", issuer = partner)]
struct SecondClient;
";

    const INTROSPECTED_TOKEN: &str =
        "#[bearer_token(client = partner_client)] token: Option<IntrospectedToken<Claims>>";

    fn introspecting_provider(parameters: &str) -> String {
        format!("{PARTNER_CLIENTS}{}", bearer_provider(parameters))
    }

    fn introspection_rejection(parameters: &str) -> String {
        rejection_for(&introspecting_provider(parameters))
    }

    #[test]
    fn binds_an_introspected_bearer_token_to_its_oauth_client() {
        assert!(matches!(
            &registries_for(&introspecting_provider(INTROSPECTED_TOKEN)).providers()[0]
                .application
                .challenge,
            AuthenticatedUserChallenge::Introspection { authorization_server }
                if authorization_server.concrete.to_string()
                    == "crate::margaret::oauth_clients::partner_client::AuthorizationServerClient"
        ));
    }

    #[test]
    fn renders_a_wrapper_that_introspects_the_bearer_token() {
        let source = wrapper_source(&registries_for(&introspecting_provider(INTROSPECTED_TOKEN)));

        assert!(source.contains(
            "pubmargaret_oauth_clients_partner_client_authorization_server_client:std::sync::Arc<margaret::framework::authorization_server_client::authorization_server_client::AuthorizationServerClient>,"
        ));
        assert!(source.contains(
            "letargument_1=matchmargaret::framework::token_introspection::introspect_bearer_token::introspect_bearer_token::<crate::Claims>(request.inputs.server.authorization(),self.margaret_oauth_clients_partner_client_authorization_server_client.as_ref(),).await.into_requirement(){margaret::framework::http::requirement::Requirement::Met(token)=>token,margaret::framework::http::requirement::Requirement::Unmet(response)=>return::std::result::Result::Ok(margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(response,),),};"
        ));
        assert!(!source.contains("route_bearer_token"));
    }

    #[test]
    fn rejects_an_introspected_bearer_token_without_a_client() {
        assert!(
            introspection_rejection(
                "#[bearer_token(issuer = partner)] token: Option<IntrospectedToken<Claims>>"
            )
            .ends_with("an IntrospectedToken names `client = <tag>`")
        );
    }

    #[test]
    fn rejects_an_introspected_bearer_token_without_its_claims() {
        assert!(
            introspection_rejection(
                "#[bearer_token(client = partner_client)] token: Option<IntrospectedToken>"
            )
            .contains("taken by value")
        );
    }

    #[test]
    fn rejects_introspected_claims_that_are_not_a_named_type() {
        assert!(
            introspection_rejection(
                "#[bearer_token(client = partner_client)] token: Option<IntrospectedToken<Vec<u8>>>"
            )
            .contains("which is not a named type without generic arguments")
        );
    }

    #[test]
    fn rejects_a_second_introspected_bearer_token() {
        assert!(
            introspection_rejection(&format!(
                "{INTROSPECTED_TOKEN}, #[bearer_token(client = second_client)] second: Option<IntrospectedToken<Claims>>"
            ))
            .contains("introspects the bearer token more than once")
        );
    }

    #[test]
    fn rejects_verified_and_introspected_bearer_tokens_in_one_provider() {
        assert!(
            introspection_rejection(&format!("{PARTNER_TOKEN}, {INTROSPECTED_TOKEN}"))
                .contains("both verifies and introspects the bearer token")
        );
        assert!(
            introspection_rejection(&format!("{INTROSPECTED_TOKEN}, {PARTNER_TOKEN}"))
                .contains("both verifies and introspects the bearer token")
        );
    }

    #[test]
    fn rejects_an_introspected_bearer_token_of_an_unplanned_oauth_client() {
        let index = IndexedSource::new(&provider_source(&introspecting_provider(
            INTROSPECTED_TOKEN,
        )))
        .index;

        assert!(matches!(
            BindingRegistries::collect(
                &index,
                ViewsAvailability::Available,
                &collected_tags(&index),
                &empty_bindings(), &[],
            ),
            Err(RequestBindingError::UnplannedOAuthClient { ref client, .. }) if client == "partner_client"
        ));
    }

    #[test]
    fn binds_a_bearer_token_to_the_trusted_issuer_of_its_oidc_issuer() {
        assert!(verifies_with(
            &registries_for(&bearer_provider(PARTNER_TOKEN)),
            "crate::margaret::trusted_issuers::partner::TrustedIssuer"
        ));
    }

    #[test]
    fn binds_a_bearer_token_to_the_trusted_issuer_of_its_jwks_endpoint() {
        assert!(verifies_with(
            &registries_for(&format!(
                "#[verifies_tokens_from_issuer(auth, audience = \"api\", issuer = \"https://auth.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Published(jwks_uri = \"https://auth.example/jwks\"))]\nstruct AuthEndpoint;\n\n{}",
                bearer_provider(
                    "#[bearer_token(issuer = auth)] token: Option<VerifiedJwt<Claims, AccessTokenProfile>>"
                )
            )),
            "crate::margaret::trusted_issuers::auth::TrustedIssuer"
        ));
    }

    #[test]
    fn routes_no_bearer_token_for_a_provider_without_a_bearer_token() {
        assert!(!wrapper_source(&registries_for(SESSION_PROVIDER)).contains("route_bearer_token"));
    }

    #[test]
    fn renders_a_wrapper_that_routes_the_bearer_token_and_admits_it_for_its_issuer() {
        let source = wrapper_source(&registries_for(&bearer_provider(&format!(
            "request: &Request, {PARTNER_TOKEN}"
        ))));
        assert!(source.contains(
            "pubmargaret_trusted_issuers_partner_trusted_issuer:std::sync::Arc<margaret::framework::trusted_issuer::trusted_issuer::TrustedIssuer>,"
        ));
        assert!(source.contains(
            "letbearer_token=matchmargaret::framework::bearer_token_verification::route_bearer_token::route_bearer_token(request.inputs.server.authorization(),&[self.margaret_trusted_issuers_partner_trusted_issuer.as_ref()],).map_err(margaret::framework::anyhow::Error::from){::std::result::Result::Ok(margaret::framework::bearer_token_verification::bearer_token_routing::BearerTokenRouting::Refused(response))=>return::std::result::Result::Ok(margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(response,),),::std::result::Result::Ok(margaret::framework::bearer_token_verification::bearer_token_routing::BearerTokenRouting::Routed(routed))=>routed,::std::result::Result::Err(error)=>return::std::result::Result::Err(error),};"
        ));
        assert!(source.contains(
            "letargument_2=matchbearer_token.admit::<crate::Claims,::margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile>(self.margaret_trusted_issuers_partner_trusted_issuer.as_ref()).await.into_requirement(){margaret::framework::http::requirement::Requirement::Met(token)=>token,margaret::framework::http::requirement::Requirement::Unmet(response)=>return::std::result::Result::Ok(margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(response,),),};"
        ));
        assert!(source.contains("self.inner.infer(argument_1,argument_2)"));
    }

    #[test]
    fn resolves_the_bearer_token_type_and_claims_through_use_statements() {
        let source = wrapper_source(&registries_for(
            "use margaret::framework::jwt_verification::access_token_profile;\nuse margaret::framework::jwt_verification::verified_jwt;\n\nmod ci {\n    mod claims {\n        pub struct Claims;\n    }\n\n    pub use claims::Claims;\n}\n\nuse crate::ci::Claims;\n\n#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\nstruct PartnerIssuer;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct RunnerProvider;\n\nimpl RunnerProvider {\n    #[infer_from_request]\n    fn infer(&self, #[bearer_token(issuer = partner)] token: Option<verified_jwt::VerifiedJwt<Claims, access_token_profile::AccessTokenProfile>>) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
        ));

        assert!(source.contains(
            "admit::<crate::ci::claims::Claims,::margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile>("
        ));
    }

    #[test]
    fn resolves_the_introspected_token_type_and_claims_through_use_statements() {
        let source = wrapper_source(&registries_for(
            "use margaret::framework::token_introspection::introspected_token;\n\nmod ci {\n    mod claims {\n        pub struct Claims;\n    }\n\n    pub use claims::Claims;\n}\n\nuse crate::ci::Claims;\n\n#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\nstruct PartnerIssuer;\n\n#[oauth_client(partner_client, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"partner\", issuer = partner)]\nstruct PartnerClient;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct RunnerProvider;\n\nimpl RunnerProvider {\n    #[infer_from_request]\n    fn infer(&self, #[bearer_token(client = partner_client)] token: Option<introspected_token::IntrospectedToken<Claims>>) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n",
        ));

        assert!(source.contains(
            "margaret::framework::token_introspection::introspect_bearer_token::introspect_bearer_token::<crate::ci::claims::Claims>("
        ));
    }

    #[test]
    fn rejects_a_verified_bearer_token_introspected_by_a_client() {
        assert!(
            rejection_for(&format!(
                "{}{}",
                bearer_provider(
                    "#[bearer_token(client = partner_client)] token: Option<VerifiedJwt<Claims, AccessTokenProfile>>"
                ),
                "#[oauth_client(partner_client, authentication = margaret::framework::oauth_vocabulary::client_authentication_method::ClientAuthenticationMethod::PrivateKeyJwt, client_id = \"partner\", issuer = partner)]\nstruct PartnerClient;\n"
            ))
            .ends_with("a VerifiedJwt names `issuer = <tag>` or `resource = <tag>`, an IntrospectedToken names `client = <tag>`")
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
            bearer_rejection(
                "#[bearer_token(issuer = partner)] token: VerifiedJwt<Claims, AccessTokenProfile>"
            )
            .contains("carries #[bearer_token] on 'VerifiedJwt < Claims , AccessTokenProfile >'")
        );
    }

    #[test]
    fn rejects_a_bearer_token_taken_by_reference() {
        assert!(
            bearer_rejection(
                "#[bearer_token(issuer = partner)] token: Option<&VerifiedJwt<Claims, AccessTokenProfile>>"
            )
            .contains("taken by value")
        );
    }

    #[test]
    fn rejects_bearer_token_claims_with_generic_arguments() {
        assert!(
            bearer_rejection(
                "#[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Vec<Claims>, AccessTokenProfile>>"
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
                "#[bearer_token(issuer = partner)] token: Option<VerifiedJwt<(Claims, Claims), AccessTokenProfile>>"
            )
            .ends_with(
                "into the claims '(Claims , Claims)', which is not a named type without generic arguments"
            )
        );
    }

    #[test]
    fn rejects_bearer_token_claims_that_match_no_type() {
        assert!(
            bearer_rejection("#[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Ghost, AccessTokenProfile>>")
                .ends_with("into the claims 'Ghost', which matches no type in scope")
        );
    }

    #[test]
    fn rejects_a_bearer_token_without_a_profile() {
        assert!(
            bearer_rejection("#[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Claims>>")
                .contains("it must be Option<margaret::framework::jwt_verification::verified_jwt::VerifiedJwt<Claims, Profile>> or Option<margaret::framework::token_introspection::introspected_token::IntrospectedToken<Claims>> taken by value")
        );
    }

    #[test]
    fn rejects_a_bearer_token_profile_with_generic_arguments() {
        assert!(
            bearer_rejection(
                "#[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Claims, Vec<AccessTokenProfile>>>"
            )
            .ends_with(
                "under the profile 'Vec < AccessTokenProfile >', which is not a named type without generic arguments"
            )
        );
    }

    #[test]
    fn rejects_a_bearer_token_profile_that_matches_no_type() {
        assert!(
            bearer_rejection(
                "#[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Claims, Ghost>>"
            )
            .ends_with("under the profile 'Ghost', which matches no type in scope")
        );
    }

    #[test]
    fn rejects_a_provider_that_admits_the_tokens_of_one_issuer_twice() {
        assert!(
            bearer_rejection(&format!(
                "{PARTNER_TOKEN}, #[bearer_token(issuer = partner)] again: Option<VerifiedJwt<Claims, AccessTokenProfile>>"
            ))
            .ends_with(
                "verifies bearer tokens of the trusted issuer 'partner' more than once; each trusted issuer admits the bearer token into exactly one argument"
            )
        );
    }

    #[test]
    fn routes_the_bearer_token_among_the_trusted_issuers_of_one_provider() {
        let source = wrapper_source(&registries_for(&format!(
            "#[verifies_tokens_from_issuer(upstream, audience = \"api\", issuer = \"https://upstream.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\nstruct UpstreamIssuer;\n\n{}",
            bearer_provider(&format!(
                "{PARTNER_TOKEN}, #[bearer_token(issuer = upstream)] upstream: Option<VerifiedJwt<Claims, IdTokenProfile>>"
            ))
            .replace(
                "use margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;",
                "use margaret::framework::jwt_verification::id_token_profile::IdTokenProfile;\nuse margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;",
            )
        )));

        assert!(source.contains(
            "&[self.margaret_trusted_issuers_partner_trusted_issuer.as_ref(),self.margaret_trusted_issuers_upstream_trusted_issuer.as_ref()],"
        ));
        assert!(source.contains(
            "letargument_2=matchbearer_token.admit::<crate::Claims,::margaret::framework::jwt_verification::id_token_profile::IdTokenProfile>(self.margaret_trusted_issuers_upstream_trusted_issuer.as_ref())"
        ));
    }

    #[test]
    fn rejects_a_bearer_token_that_also_carries_a_form_request() {
        assert!(
            bearer_rejection(
                "#[bearer_token(issuer = partner)] #[form_request(from = RequestInput::Query)] token: Option<VerifiedJwt<Claims, AccessTokenProfile>>"
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
    fn rejects_a_bearer_token_whose_trusted_issuer_the_container_does_not_plan() {
        let index = IndexedSource::new(&provider_source(&bearer_provider(PARTNER_TOKEN))).index;
        let rejection = BindingRegistries::collect(
            &index,
            ViewsAvailability::Available,
            &collected_tags(&index),
            &empty_bindings(),
            &[],
        )
        .err()
        .expect("the binding registries are rejected")
        .to_string();

        assert!(rejection.ends_with(
            "verifies the bearer token of the trusted issuer 'partner', which the container does not plan"
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
        let route_path = RoutePath::parse("/").expect("the route path is routable");
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
        let extraction: String = render_head_extractions(
            &parameters,
            &CapturedProviders::capture(&parameters, &mut NameAllocator::new()),
            &HeadExtractionContext {
                continuation_return: &quote::quote! { return response },
                error_return: &quote::quote! { return error },
                owner: &proc_macro2::TokenStream::new(),
                request_local: &format_ident!("request"),
            },
        )
        .to_string()
        .split_whitespace()
        .collect();

        assert!(extraction.contains(
            "margaret::framework::identity::require_bearer_authenticated_user::require_bearer_authenticated_user(outcome)"
        ));
    }

    #[test]
    fn binds_a_parameter_to_a_local_named_after_its_position() {
        let indexed = IndexedSource::new(&provider_source(&format!(
            "{}\nstruct Page;\n\nimpl Page {{\n    #[process]\n    fn respond(&self, #[authenticated_user] _caller: User) -> anyhow::Result<Response> {{}}\n}}\n",
            bearer_provider(PARTNER_TOKEN)
        )));
        let registries = collect_registries(&indexed.index, ViewsAvailability::Available)
            .expect("the binding registries are collected");
        let item = indexed.item("Page");
        let route_path = RoutePath::parse("/").expect("the route path is routable");
        let holders: Vec<String> = classify_parameters(
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
        .expect("the responder binds")
        .iter()
        .map(|parameter| parameter.holder.to_string())
        .collect();

        assert_eq!(holders, ["argument_1"]);
    }

    #[test]
    fn refuses_to_inject_a_trusted_issuer_into_a_handshake() {
        let indexed = IndexedSource::new(
            "struct Room;\n\nimpl Room {\n    #[process]\n    fn build(trusted_issuer: std::sync::Arc<crate::margaret::trusted_issuers::partner::TrustedIssuer>) -> anyhow::Result<Self> {}\n}\n",
        );
        let item = indexed.item("Room");
        let route_path = RoutePath::parse("/room").expect("the route path is routable");
        let bindings = trusted_issuer_bindings();
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
                "{}\nstruct Runner;\n\nstruct Admin;\n\n#[singleton]\n#[infers_authenticated_user(user_model = Admin)]\nstruct AdminProvider;\n\nimpl AdminProvider {{\n    #[infer_from_request]\n    fn infer(&self, #[bearer_token(issuer = partner)] token: Option<VerifiedJwt<Claims, AccessTokenProfile>>) -> anyhow::Result<AuthenticatedUserOutcome<Admin>> {{}}\n}}\n\nstruct Page;\n\nimpl Page {{\n    #[process]\n    fn respond(&self, #[authenticated_user] runner: Runner, #[authenticated_user] admin: Admin) -> anyhow::Result<Response> {{}}\n}}\n",
                bearer_provider(PARTNER_TOKEN).replace("User", "Runner")
            )),
            "responder 'Page' infers more than one authenticated user from the bearer token; a request presents one bearer credential, so exactly one #[infers_authenticated_user] provider verifies it"
        );
    }

    #[test]
    fn collects_the_console_arguments_of_the_trusted_issuer_a_bearer_provider_verifies() {
        let index = IndexedSource::new(&provider_source(&format!(
            "use margaret::framework::jwt_verification::access_token_profile::AccessTokenProfile;\nuse margaret::framework::jwt_verification::verified_jwt::VerifiedJwt;\n\nstruct Claims;\n\n#[verifies_tokens_from_issuer(partner, audience = \"api\", issuer = \"https://partner.example\", keys = margaret::framework::trusted_issuer::issuer_keys::IssuerKeys::Discovered)]\nstruct PartnerIssuer;\n\n#[singleton]\nstruct PartnerSettings;\n\nimpl PartnerSettings {{\n    #[constructor]\n    fn create(#[console_argument(from = \"audience\")] audience: String) -> anyhow::Result<Self> {{}}\n}}\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct RunnerProvider;\n\nimpl RunnerProvider {{\n    #[infer_from_request]\n    fn infer(&self, {PARTNER_TOKEN}) -> anyhow::Result<AuthenticatedUserOutcome<User>> {{}}\n}}\n"
        )))
        .index;
        let tags = collected_tags(&index);
        let bindings = render_container(
            &index,
            &scan(&index).expect("the console arguments are scanned"),
            &[FrameworkProvider {
                construction: FrameworkConstruction::Constructor {
                    dependencies: vec![FrameworkDependency::SingletonView(CanonicalPath::new(
                        vec!["crate".to_string(), "PartnerSettings".to_string()],
                    ))],
                    is_async: false,
                    method: "create".to_string(),
                    outcome: ConstructorOutcome::Infallible,
                },
                ..trusted_issuer_provider("partner")
            }],
            &DeclaredPostgresDatabase::Absent,
            &DeclaredTokenIssuance::Absent,
        )
        .expect("the container renders")
        .bindings;
        let registries =
            BindingRegistries::collect(&index, ViewsAvailability::Available, &tags, &bindings, &[])
                .expect("the binding registries are collected");
        let roots: Vec<CanonicalPath> = binding_roots(&RequestBinding::AuthenticatedUser {
            application: registries.providers()[0].application.clone(),
            requirement: AuthenticatedUserRequirement::Required,
        })
        .into_iter()
        .cloned()
        .collect();
        let names: Vec<String> = bindings
            .serve_inputs(&roots)
            .expect("the trusted issuer has planned console arguments")
            .iter()
            .map(|argument| argument.input.name().to_string())
            .collect();

        assert_eq!(names, vec!["audience".to_string()]);
    }

    #[test]
    fn rejects_a_provider_that_reads_the_request_body() {
        assert!(
            rejection_for(
                "use margaret::framework::http_uploaded_file::uploaded_files::UploadedFiles;\n\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct SessionUserProvider;\n\nimpl SessionUserProvider {\n    #[infer_from_request]\n    fn infer(&self, files: UploadedFiles) -> anyhow::Result<AuthenticatedUserOutcome<User>> {}\n}\n"
            )
            .contains("which only an HTTP responder receives")
        );
    }

    const ISSUED_SESSIONS: &str = "\
use margaret::framework::sessions::session::Session;

#[issues_tokens(provider, issuer = \"https://issuer.example\")]
struct ProviderIssuance;

#[issues_sessions(issuer = provider, audience = \"browser\", cookies = margaret::framework::sessions::session_cookies::SessionCookies::HostOnly)]
struct BrowserSessions;
";

    const CONSUMED_SESSIONS: &str = "\
use margaret::framework::sessions::session::Session;

#[consumes_sessions(issuer = partner, cookie_domain_from = \"SESSION_COOKIE_DOMAIN\", refresh_url_from = \"SESSION_REFRESH_URL\")]
struct PartnerSessions;
";

    const ISSUED_SESSION: &str = "#[session(issuer = provider)] session: Option<Session>";

    fn reader_provider(declarations: &str, parameters: &str) -> String {
        format!(
            "{declarations}\n#[singleton]\n#[infers_authenticated_user(user_model = User)]\nstruct ReaderProvider;\n\nimpl ReaderProvider {{\n    #[infer_from_request]\n    fn infer(&self, {parameters}) -> anyhow::Result<AuthenticatedUserOutcome<User>> {{}}\n}}\n"
        )
    }

    #[test]
    fn challenges_a_user_read_from_issued_sessions_through_them() {
        assert!(matches!(
            registries_for(&reader_provider(ISSUED_SESSIONS, ISSUED_SESSION)).providers()[0]
                .application
                .challenge,
            AuthenticatedUserChallenge::Session { source, .. } if source == SessionSource::Issued
        ));
    }

    #[test]
    fn challenges_a_user_read_from_consumed_sessions_through_them() {
        assert!(matches!(
            registries_for(&reader_provider(
                &format!("{PARTNER_ISSUER}{CONSUMED_SESSIONS}"),
                "#[session(issuer = partner)] session: Option<Session>",
            ))
            .providers()[0]
                .application
                .challenge,
            AuthenticatedUserChallenge::Session { source, .. } if source == SessionSource::Consumed
        ));
    }

    #[test]
    fn renders_a_wrapper_that_resolves_issued_sessions_before_inferring_the_user() {
        let source = wrapper_source(&registries_for(&reader_provider(
            ISSUED_SESSIONS,
            ISSUED_SESSION,
        )));

        assert!(source.contains(
            "implmargaret::framework::identity::infers_session_user::InfersSessionUserforReaderProvider"
        ));
        assert!(source.contains(
            "letmargaret::framework::sessions::resolved_session::ResolvedSession{cookie_changes:changed_cookies,session:argument_1,}=self.margaret_sessions_issued_sessions.resolve(request).await?;"
        ));
        assert!(source.contains(
            "self.inner.infer(argument_1).map(|outcome|{margaret::framework::identity::session_user_inference::SessionUserInference{cookie_changes:changed_cookies,outcome,}})"
        ));
    }

    #[test]
    fn renders_a_wrapper_that_interrupts_on_unavailable_consumed_sessions() {
        let source = wrapper_source(&registries_for(&reader_provider(
            &format!("{PARTNER_ISSUER}{CONSUMED_SESSIONS}"),
            "#[session(issuer = partner)] session: Option<Session>",
        )));

        assert!(source.contains(
            "margaret::framework::sessions::session_resolution::SessionResolution::Unavailable(unavailability)=>{return::std::result::Result::Ok(margaret::framework::identity::session_user_inference::SessionUserInference{cookie_changes:margaret::framework::http::cookie_changes::CookieChanges{cookies:::std::vec::Vec::new(),},outcome:margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(margaret::framework::http::response_continuation::ResponseContinuation::from(unavailability),),},);}"
        ));
    }

    #[test]
    fn renders_a_wrapper_that_keeps_the_session_cookie_changes_on_an_interruption() {
        let source = wrapper_source(&registries_for(&reader_provider(
            ISSUED_SESSIONS,
            &format!(
                "{ISSUED_SESSION}, #[form_request(from = margaret::framework::http_validation::request_input::RequestInput::Query)] filter: Filter"
            ),
        )));

        assert!(source.contains(
            "return::std::result::Result::Ok(margaret::framework::identity::session_user_inference::SessionUserInference{cookie_changes:changed_cookies,outcome:margaret::framework::identity::authenticated_user_outcome::AuthenticatedUserOutcome::Interrupted(response,),},)"
        ));
    }

    #[test]
    fn renders_the_session_user_of_a_responder_with_its_cookie_changes() {
        let indexed = IndexedSource::new(&provider_source(&format!(
            "{}\nstruct Page;\n\nimpl Page {{\n    #[process]\n    fn respond(&self, #[authenticated_user] reader: User) -> anyhow::Result<Response> {{}}\n}}\n",
            reader_provider(ISSUED_SESSIONS, ISSUED_SESSION)
        )));
        let registries = collect_registries(&indexed.index, ViewsAvailability::Available)
            .expect("the binding registries are collected");
        let item = indexed.item("Page");
        let route_path = RoutePath::parse("/").expect("the route path is routable");
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
        let captured = CapturedProviders::capture(&parameters, &mut NameAllocator::new());
        let context = HeadExtractionContext {
            continuation_return: &quote::quote! { return response },
            error_return: &quote::quote! { return error },
            owner: &proc_macro2::TokenStream::new(),
            request_local: &format_ident!("request"),
        };
        let inference: String = render_session_user_inference(
            session_user_parameter(&parameters).expect("the responder reads a session user"),
            &captured,
            &format_ident!("changed_cookies"),
            &context,
        )
        .to_string()
        .split_whitespace()
        .collect();
        let extraction: String = render_head_extractions(&parameters, &captured, &context)
            .to_string()
            .split_whitespace()
            .collect();

        assert!(inference.contains(
            "letmargaret::framework::identity::session_user_inference::SessionUserInference{cookie_changes:changed_cookies,outcome:argument_1,}=matchmargaret::framework::identity::infers_session_user::InfersSessionUser::infer("
        ));
        assert!(!extraction.contains("SessionUserInference"));
        assert!(extraction.contains(
            "margaret::framework::http::requirement::Requirement::Unmet(response)=>returnresponse"
        ));
    }

    #[test]
    fn rejects_a_session_outside_an_inference_method() {
        assert!(
            responder_rejection(&format!(
                "{ISSUED_SESSIONS}\nstruct Page;\n\nimpl Page {{\n    #[process]\n    fn respond(&self, {ISSUED_SESSION}) -> anyhow::Result<Response> {{}}\n}}\n"
            ))
            .ends_with("carries #[session], which is only available in an #[infer_from_request] method of an #[infers_authenticated_user] provider")
        );
    }

    #[test]
    fn rejects_a_session_of_another_type() {
        assert!(
            rejection_for(&reader_provider(
                ISSUED_SESSIONS,
                "#[session(issuer = provider)] session: Session"
            ))
            .ends_with("carries #[session] on 'Session'; it must be Option<margaret::framework::sessions::session::Session> taken by value")
        );
    }

    #[test]
    fn rejects_a_session_with_another_marker() {
        assert!(
            rejection_for(&reader_provider(
                ISSUED_SESSIONS,
                "#[session(issuer = provider)] #[form_request(from = margaret::framework::http_validation::request_input::RequestInput::Query)] session: Option<Session>"
            ))
            .ends_with("carries #[session] together with another argument marker; an argument may use at most one")
        );
    }

    #[test]
    fn rejects_a_session_on_the_peer_spiffe_id() {
        assert!(
            rejection_for(&reader_provider(
                ISSUED_SESSIONS,
                "#[session(issuer = provider)] peer: &spiffe::spiffe_id::SpiffeId"
            ))
            .contains("is the peer SPIFFE id and must not also carry")
        );
    }

    #[test]
    fn rejects_a_session_read_twice() {
        assert!(
            rejection_for(&reader_provider(
                ISSUED_SESSIONS,
                &format!("{ISSUED_SESSION}, #[session(issuer = provider)] again: Option<Session>")
            ))
            .ends_with(
                "reads the session more than once; exactly one argument may carry #[session]"
            )
        );
    }

    #[test]
    fn rejects_a_session_read_after_a_bearer_token() {
        assert!(
            rejection_for(&reader_provider(
                &format!("{PARTNER_ISSUER}{ISSUED_SESSIONS}"),
                &format!("{PARTNER_TOKEN}, {ISSUED_SESSION}")
            ))
            .ends_with("reads both a session and a bearer token; a provider infers its user either from the session cookies or from a bearer credential")
        );
    }

    #[test]
    fn rejects_a_bearer_token_verified_after_a_session() {
        assert!(
            rejection_for(&reader_provider(
                &format!("{PARTNER_ISSUER}{ISSUED_SESSIONS}"),
                &format!("{ISSUED_SESSION}, {PARTNER_TOKEN}")
            ))
            .ends_with("reads both a session and a bearer token; a provider infers its user either from the session cookies or from a bearer credential")
        );
    }

    #[test]
    fn rejects_a_bearer_token_introspected_after_a_session() {
        assert!(
            rejection_for(&reader_provider(
                &format!("{PARTNER_ISSUER}{PARTNER_CLIENTS}{ISSUED_SESSIONS}"),
                &format!("{ISSUED_SESSION}, {INTROSPECTED_TOKEN}")
            ))
            .ends_with("reads both a session and a bearer token; a provider infers its user either from the session cookies or from a bearer credential")
        );
    }

    #[test]
    fn rejects_sessions_the_container_does_not_plan() {
        let index = IndexedSource::new(&provider_source(&reader_provider(
            ISSUED_SESSIONS,
            ISSUED_SESSION,
        )))
        .index;

        assert!(
            BindingRegistries::collect(
                &index,
                ViewsAvailability::Available,
                &collected_tags(&index),
                &empty_bindings(),
                &[],
            )
            .err()
            .expect("the unplanned sessions are rejected")
            .to_string()
            .ends_with("reads a session, but the dependency container plans no 'crate::margaret::sessions::IssuedSessions'")
        );
    }

    #[test]
    fn rejects_two_session_users_of_one_responder() {
        let second = "#[singleton]\n#[infers_authenticated_user(user_model = Editor)]\nstruct EditorProvider;\n\nimpl EditorProvider {\n    #[infer_from_request]\n    fn infer(&self, #[session(issuer = provider)] session: Option<Session>) -> anyhow::Result<AuthenticatedUserOutcome<Editor>> {}\n}\nstruct Editor;\n";
        let index = IndexedSource::new(&provider_source(&format!(
            "{}{second}\nstruct Page;\n\nimpl Page {{\n    #[process]\n    fn respond(&self, #[authenticated_user] reader: User, #[authenticated_user] editor: Editor) -> anyhow::Result<Response> {{}}\n}}\n",
            reader_provider(ISSUED_SESSIONS, ISSUED_SESSION)
        )))
        .index;
        let registries = collect_registries(&index, ViewsAvailability::Available)
            .expect("the binding registries are collected");
        let route_path = RoutePath::parse("/").expect("the route path is routable");
        let item = index
            .item(&CanonicalPath::new(vec![
                "crate".to_string(),
                "Page".to_string(),
            ]))
            .expect("the responder is indexed");

        assert!(
            classify_parameters(
                &index,
                item,
                process_method(item).expect("the responder has a #[process] method"),
                &BindingContext::Responder {
                    route_path: &route_path,
                    server: "public",
                    subject: "responder 'Page'",
                },
                &registries,
            )
            .err()
            .expect("the second session user is rejected")
            .to_string()
            .ends_with("infers more than one authenticated user from the session; a request resolves its session once, so exactly one #[infers_authenticated_user] provider reads it")
        );
    }
}
