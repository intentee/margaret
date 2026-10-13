use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::format_ident;
use quote::quote;

use margaret_attributes::struct_shape::StructShape;
use margaret_codegen_tokens::bootstrap_argument_field_ident::bootstrap_argument_field_ident;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::positional_naming::PositionalNaming;
use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;

use crate::bootstrap_arguments_module::bootstrap_arguments_module;
use crate::bootstrap_arguments_type::bootstrap_arguments_type;
use crate::construct_singleton_path::construct_singleton_path;
use crate::construction_error_path::construction_error_path;
use crate::construction_errors_doc::construction_errors_doc;
use crate::construction_flow::construction_flow;
use crate::constructor_outcome::ConstructorOutcome;
use crate::container_field_ident::container_field_ident;
use crate::container_plan::ContainerPlan;
use crate::direct_construction::DirectConstruction;
use crate::field_ident::field_ident;
use crate::field_type::field_type;
use crate::planned_dependency::PlannedDependency;
use crate::planned_field::PlannedField;
use crate::planned_provider::PlannedProvider;
use crate::planned_url::PlannedUrl;
use crate::reverse_serve_input_weaver::ReverseServeInputWeaver;
use crate::served_container::ServedContainer;
use crate::slotted_serve_input::SlottedServeInput;
use crate::unify_serve_inputs::unify_serve_inputs;

fn build_module() -> ModulePath {
    ModulePath::root("container").child("build")
}

fn fieldless_literal(concrete: &TokenStream, shape: StructShape) -> TokenStream {
    match shape {
        StructShape::Named { .. } => quote! { #concrete {} },
        StructShape::Unit => quote! { #concrete },
        StructShape::Unnamed { .. } => quote! { #concrete() },
    }
}

fn field_expression(
    PlannedField {
        concrete_path,
        field_name,
    }: &PlannedField,
    arc_clone: ArcClone,
) -> TokenStream {
    let dependency = format_ident!("{field_name}");

    match arc_clone {
        ArcClone::Inferred => quote! { ::std::sync::Arc::clone(&#dependency) },
        ArcClone::Typed => {
            let value_type = path_tokens(concrete_path);

            quote! { ::std::sync::Arc::<#value_type>::clone(&#dependency) }
        }
    }
}

fn dependency_expression(
    dependency: &PlannedDependency,
    weaver: &mut ReverseServeInputWeaver,
    arc_clone: ArcClone,
) -> TokenStream {
    match dependency {
        PlannedDependency::Collection(fields) => {
            let elements = fields
                .iter()
                .map(|field| field_expression(field, arc_clone));

            quote! { ::std::vec::Vec::from([#(#elements),*]) }
        }
        PlannedDependency::Constant(path) => path_tokens(path),
        PlannedDependency::ServeInput { input, slot } => weaver.weave(input, *slot),
        PlannedDependency::Single(field) => field_expression(field, arc_clone),
        PlannedDependency::Urls(urls) => {
            let mut elements: Vec<TokenStream> = urls
                .iter()
                .rev()
                .map(|url| match url {
                    PlannedUrl::Declared(url) => quote! { ::std::string::String::from(#url) },
                    PlannedUrl::Route { input, slot } => {
                        let route_url = weaver.weave(input, *slot);

                        quote! { ::std::string::String::from(#route_url) }
                    }
                })
                .collect();

            elements.reverse();

            quote! { ::std::vec::Vec::from([#(#elements),*]) }
        }
    }
}

fn dependency_expressions(
    dependencies: &[PlannedDependency],
    weaver: &mut ReverseServeInputWeaver,
    arc_clone: ArcClone,
) -> Vec<TokenStream> {
    let mut expressions: Vec<TokenStream> = dependencies
        .iter()
        .rev()
        .map(|dependency| dependency_expression(dependency, weaver, arc_clone))
        .collect();

    expressions.reverse();
    expressions
}

fn constructor_call(
    concrete: &TokenStream,
    method: &str,
    arguments: &[TokenStream],
    is_async: bool,
) -> TokenStream {
    let constructor = format_ident!("{method}");
    let call = quote! { #concrete::#constructor(#(#arguments),*) };

    if is_async {
        quote! { #call.await }
    } else {
        call
    }
}

fn direct_value(planned: &PlannedProvider, weaver: &mut ReverseServeInputWeaver) -> TokenStream {
    let provider = &planned.provider;
    let concrete = path_tokens(&provider.concrete_path);
    let dependencies = &planned.dependencies;

    match &provider.construction {
        DirectConstruction::Constructor {
            is_async,
            method,
            outcome,
            ..
        } => {
            let call = constructor_call(
                &concrete,
                method,
                &dependency_expressions(dependencies, weaver, ArcClone::Inferred),
                *is_async,
            );

            match outcome {
                ConstructorOutcome::Fallible => {
                    let construct_singleton = construct_singleton_path();
                    let singleton = provider.concrete_path.to_string();

                    quote! { #construct_singleton(#singleton, #call)? }
                }
                ConstructorOutcome::Infallible => call,
            }
        }
        DirectConstruction::Fieldless { shape } => fieldless_literal(&concrete, *shape),
        DirectConstruction::FrameworkConstructor {
            is_async,
            method,
            outcome,
            ..
        } => {
            let call = constructor_call(
                &concrete,
                method,
                &dependency_expressions(dependencies, weaver, ArcClone::Typed),
                *is_async,
            );

            match outcome {
                ConstructorOutcome::Fallible => {
                    let construct_singleton = construct_singleton_path();
                    let singleton = provider.concrete_path.to_string();

                    quote! {
                        #construct_singleton(
                            #singleton,
                            #call.map_err(margaret::framework::anyhow::Error::from),
                        )?
                    }
                }
                ConstructorOutcome::Infallible => call,
            }
        }
        DirectConstruction::FrameworkPrepared { .. } => {
            let prepared = dependency_expressions(dependencies, weaver, ArcClone::Inferred);

            quote! { #(#prepared)* }
        }
        DirectConstruction::FrameworkUnit => quote! { #concrete },
        DirectConstruction::FrameworkAccessor { accessor, .. } => {
            let accessor = format_ident!("{accessor}");
            let sources = dependency_expressions(dependencies, weaver, ArcClone::Inferred);

            quote! { #(#sources)*.#accessor() }
        }
    }
}

fn constructed_value(planned: &PlannedProvider, weaver: &mut ReverseServeInputWeaver) -> FlowValue {
    let provider = &planned.provider;
    let value = direct_value(planned, weaver);

    FlowValue {
        binding: field_ident(provider),
        value: match &provider.construction {
            DirectConstruction::FrameworkAccessor { .. }
            | DirectConstruction::Constructor {
                outcome: ConstructorOutcome::Fallible,
                ..
            }
            | DirectConstruction::FrameworkConstructor {
                outcome: ConstructorOutcome::Fallible,
                ..
            } => value,
            DirectConstruction::Fieldless { .. }
            | DirectConstruction::Constructor {
                outcome: ConstructorOutcome::Infallible,
                ..
            }
            | DirectConstruction::FrameworkConstructor {
                outcome: ConstructorOutcome::Infallible,
                ..
            }
            | DirectConstruction::FrameworkPrepared { .. }
            | DirectConstruction::FrameworkUnit => quote! { ::std::sync::Arc::new(#value) },
        },
    }
}

fn arguments_declaration(
    function: &Ident,
    inputs: &[SlottedServeInput],
    naming: PositionalNaming,
) -> TokenStream {
    if inputs.is_empty() {
        return TokenStream::new();
    }

    let module = bootstrap_arguments_module(function);
    let arguments_type = bootstrap_arguments_type(function);
    let bindings = inputs.iter().map(|SlottedServeInput { slot, .. }| {
        let field = bootstrap_argument_field_ident(*slot);
        let binding = naming.ident(*slot);

        quote! { #field: #binding, }
    });

    quote! {
        super::#module::#arguments_type { #(#bindings)* }: super::#module::#arguments_type
    }
}

fn arguments_module(
    function: &Ident,
    inputs: &[SlottedServeInput],
) -> Option<GeneratedModuleTokens> {
    if inputs.is_empty() {
        return None;
    }

    let arguments_type = bootstrap_arguments_type(function);
    let fields = inputs.iter().map(|SlottedServeInput { input, slot }| {
        let field = bootstrap_argument_field_ident(*slot);
        let value_type = input.field_type();

        quote! { pub #field: #value_type, }
    });

    Some(GeneratedModuleTokens::new(
        build_module().child(bootstrap_arguments_module(function).to_string()),
        quote! {
            pub struct #arguments_type {
                #(#fields)*
            }
        },
    ))
}

fn flow_values(flow: &[&PlannedProvider], weaver: &mut ReverseServeInputWeaver) -> Vec<FlowValue> {
    let mut values: Vec<FlowValue> = flow
        .iter()
        .rev()
        .map(|planned| constructed_value(planned, weaver))
        .collect();

    values.reverse();
    values
}

fn root_builder(root: &PlannedProvider, plan: &ContainerPlan) -> RootBuilder {
    let dependencies: Vec<&PlannedProvider> = construction_flow(plan, &[root])
        .into_iter()
        .filter(|planned| planned.key != root.key)
        .collect();
    let provider = &root.provider;
    let function = format_ident!("construct_{}", provider.field_name);
    let naming = plan.serve_input_naming();
    let parameters = arguments_declaration(&function, &root.serve_inputs, naming);
    let mut weaver = ReverseServeInputWeaver::new(naming);
    let root_value = constructed_value(root, &mut weaver);
    let mut statements: Vec<TokenStream> = flow_values(&dependencies, &mut weaver)
        .iter()
        .map(FlowValue::statement)
        .collect();
    let constructed = match root.outcome {
        ConstructorOutcome::Fallible => {
            statements.push(root_value.statement());

            root_value.binding.to_token_stream()
        }
        ConstructorOutcome::Infallible => root_value.value,
    };
    let tokens = builder_function(
        root.is_async,
        &function,
        &parameters,
        constructed_signature(
            root.outcome,
            root.is_async,
            &statements,
            &constructed,
            &field_type(provider),
        ),
    );

    RootBuilder {
        arguments: arguments_module(&function, &root.serve_inputs),
        function,
        tokens,
    }
}

fn serve_builder(plan: &ContainerPlan, served_roots: &[&PlannedProvider]) -> RootBuilder {
    let flow = construction_flow(plan, served_roots);
    let inputs = unify_serve_inputs(
        served_roots
            .iter()
            .flat_map(|root| root.serve_inputs.iter()),
    );
    let function = format_ident!("serve");
    let naming = plan.serve_input_naming();
    let parameters = arguments_declaration(&function, &inputs, naming);
    let statements: Vec<TokenStream> =
        flow_values(&flow, &mut ReverseServeInputWeaver::new(naming))
            .iter()
            .map(FlowValue::statement)
            .collect();
    let fields = served_roots.iter().enumerate().map(|(position, planned)| {
        let binding = field_ident(&planned.provider);
        let field = container_field_ident(position);

        quote! { #field: #binding, }
    });
    let is_async = served_roots.iter().any(|entry| entry.is_async);
    let tokens = builder_function(
        is_async,
        &function,
        &parameters,
        constructed_signature(
            served_roots
                .iter()
                .fold(ConstructorOutcome::Infallible, |outcome, root| {
                    outcome.combined_with(root.outcome)
                }),
            is_async,
            &statements,
            &quote! {
                super::super::Container {
                    #(#fields)*
                }
            },
            &quote! { super::super::Container },
        ),
    );

    RootBuilder {
        arguments: arguments_module(&function, &inputs),
        function,
        tokens,
    }
}

fn builder_function(
    is_async: bool,
    function: &Ident,
    parameters: &TokenStream,
    ConstructedSignature {
        attributes,
        body,
        return_type,
    }: ConstructedSignature,
) -> TokenStream {
    let asyncness = is_async.then(|| quote! { async });
    let too_many_lines = too_many_lines_allow();

    quote! {
        #attributes
        #too_many_lines
        pub #asyncness fn #function(#parameters) -> #return_type {
            #body
        }
    }
}

fn constructed_signature(
    outcome: ConstructorOutcome,
    is_async: bool,
    statements: &[TokenStream],
    constructed: &TokenStream,
    constructed_type: &TokenStream,
) -> ConstructedSignature {
    match outcome {
        ConstructorOutcome::Fallible => {
            let error = construction_error_path();

            ConstructedSignature {
                attributes: construction_errors_doc(),
                body: quote! {
                    #(#statements)*

                    Ok(#constructed)
                },
                return_type: quote! { ::std::result::Result<#constructed_type, #error> },
            }
        }
        ConstructorOutcome::Infallible => ConstructedSignature {
            attributes: if is_async {
                TokenStream::new()
            } else {
                quote! { #[must_use] }
            },
            body: quote! {
                #(#statements)*

                #constructed
            },
            return_type: constructed_type.clone(),
        },
    }
}

#[derive(Clone, Copy)]
enum ArcClone {
    Inferred,
    Typed,
}

struct FlowValue {
    binding: Ident,
    value: TokenStream,
}

impl FlowValue {
    fn statement(&self) -> TokenStream {
        let Self { binding, value } = self;

        quote! { let #binding = #value; }
    }
}

struct ConstructedSignature {
    attributes: TokenStream,
    body: TokenStream,
    return_type: TokenStream,
}

struct RootBuilder {
    arguments: Option<GeneratedModuleTokens>,
    function: Ident,
    tokens: TokenStream,
}

pub(crate) fn render_build(
    plan: &ContainerPlan,
    served: &ServedContainer,
    builder_roots: &[&PlannedProvider],
) -> Vec<GeneratedModuleTokens> {
    let served_builder = match served {
        ServedContainer::Served { roots } => Some(serve_builder(plan, roots)),
        ServedContainer::Unserved => None,
    };
    let builders = builder_roots
        .iter()
        .map(|root| root_builder(root, plan))
        .chain(served_builder)
        .collect::<Vec<RootBuilder>>();
    let module_declarations = builders.iter().map(|builder| {
        let function = &builder.function;
        let arguments_module = builder
            .arguments
            .as_ref()
            .map(|_| bootstrap_arguments_module(function))
            .map(|module| quote! { pub mod #module; });

        quote! {
            #arguments_module
            mod #function;
        }
    });
    let module_exports = builders.iter().map(|builder| {
        let function = &builder.function;

        quote! { pub use #function::#function; }
    });
    let build = build_module();
    let mut modules = vec![GeneratedModuleTokens::new(
        build.clone(),
        quote! {
            #(#module_declarations)*

            #(#module_exports)*
        },
    )];

    for builder in builders {
        modules.extend(builder.arguments);
        modules.push(GeneratedModuleTokens::new(
            build.child(builder.function.to_string()),
            builder.tokens,
        ));
    }

    modules
}
