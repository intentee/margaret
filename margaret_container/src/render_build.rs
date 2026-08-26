use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::struct_shape::StructShape;
use margaret_codegen_tokens::console_argument_field_ident::console_argument_field_ident;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_codegen_tokens::too_many_lines_allow::too_many_lines_allow;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::bootstrap_arguments_module::bootstrap_arguments_module;
use crate::bootstrap_arguments_type::bootstrap_arguments_type;
use crate::construct_singleton_path::construct_singleton_path;
use crate::construction_error_path::construction_error_path;
use crate::construction_errors_doc::construction_errors_doc;
use crate::construction_flow::construction_flow;
use crate::container_field_ident::container_field_ident;
use crate::container_plan::ContainerPlan;
use crate::direct_construction::DirectConstruction;
use crate::field_ident::field_ident;
use crate::field_type::field_type;
use crate::planned_dependency::PlannedDependency;
use crate::planned_provider::PlannedProvider;
use crate::reverse_console_argument_weaver::ReverseConsoleArgumentWeaver;
use crate::serve_console_arguments::ServeConsoleArguments;

fn fieldless_literal(concrete: &TokenStream, shape: StructShape) -> TokenStream {
    match shape {
        StructShape::Named { .. } => quote! { #concrete {} },
        StructShape::Unit => quote! { #concrete },
        StructShape::Unnamed { .. } => quote! { #concrete() },
    }
}

fn dependency_expression(
    dependency: &PlannedDependency,
    weaver: &mut ReverseConsoleArgumentWeaver,
) -> TokenStream {
    match dependency {
        PlannedDependency::ConsoleArgument { argument, slot } => weaver.weave(argument, *slot),
        PlannedDependency::Single { field_name } => {
            let dependency = format_ident!("{field_name}");

            quote! { ::std::sync::Arc::clone(&#dependency) }
        }
    }
}

fn dependency_expressions(
    dependencies: &[PlannedDependency],
    weaver: &mut ReverseConsoleArgumentWeaver,
) -> Vec<TokenStream> {
    let mut expressions: Vec<TokenStream> = dependencies
        .iter()
        .rev()
        .map(|dependency| dependency_expression(dependency, weaver))
        .collect();

    expressions.reverse();
    expressions
}

fn direct_value(
    planned: &PlannedProvider,
    weaver: &mut ReverseConsoleArgumentWeaver,
) -> TokenStream {
    let provider = &planned.provider;
    let concrete = path_tokens(&provider.concrete_path);
    let dependencies = &planned.dependencies;

    match &provider.construction {
        DirectConstruction::Constructor {
            is_async, method, ..
        } => {
            let constructor = format_ident!("{method}");
            let arguments = dependency_expressions(dependencies, weaver);
            let construct_singleton = construct_singleton_path();
            let singleton = provider.concrete_path.to_string();
            let call = if *is_async {
                quote! { #concrete::#constructor(#(#arguments),*).await }
            } else {
                quote! { #concrete::#constructor(#(#arguments),*) }
            };

            quote! { #construct_singleton(#singleton, #call)? }
        }
        DirectConstruction::Fieldless { shape } => fieldless_literal(&concrete, *shape),
        DirectConstruction::FrameworkConstructor {
            is_async, method, ..
        } => {
            let constructor = format_ident!("{method}");
            let arguments = dependency_expressions(dependencies, weaver);

            if *is_async {
                quote! { #concrete::#constructor(#(#arguments),*).await }
            } else {
                quote! { #concrete::#constructor(#(#arguments),*) }
            }
        }
        DirectConstruction::FrameworkUnit => quote! { #concrete },
        DirectConstruction::FrameworkAccessor { accessor, .. } => {
            let accessor = format_ident!("{accessor}");
            let sources = dependency_expressions(dependencies, weaver);

            quote! { #(#sources)*.#accessor() }
        }
        DirectConstruction::Resolved { resolver, .. } => {
            let resolver = path_tokens(resolver);
            let arguments = dependency_expressions(dependencies, weaver);

            quote! { #resolver(#(#arguments),*) }
        }
    }
}

fn statement(planned: &PlannedProvider, weaver: &mut ReverseConsoleArgumentWeaver) -> TokenStream {
    let provider = &planned.provider;
    let binding = field_ident(provider);
    let declared_type = provider.provided.is_trait_object().then(|| {
        let value_type = field_type(provider);

        quote! { : #value_type }
    });
    let value = direct_value(planned, weaver);
    let constructed = match &provider.construction {
        DirectConstruction::Constructor { .. }
        | DirectConstruction::FrameworkAccessor { .. }
        | DirectConstruction::Resolved { .. } => value,
        DirectConstruction::Fieldless { .. }
        | DirectConstruction::FrameworkConstructor { .. }
        | DirectConstruction::FrameworkUnit => quote! { ::std::sync::Arc::new(#value) },
    };

    quote! { let #binding #declared_type = #constructed; }
}

struct RootBuilder {
    arguments: Option<GeneratedModuleTokens>,
    function: Ident,
    tokens: TokenStream,
}

fn arguments_declaration(function: &Ident, slots: &[usize]) -> TokenStream {
    if slots.is_empty() {
        return TokenStream::new();
    }

    let module = bootstrap_arguments_module(function);
    let arguments_type = bootstrap_arguments_type(function);
    let bindings = slots.iter().map(|slot| {
        let field = console_argument_field_ident(*slot);
        let binding = console_argument_ident(*slot);

        quote! { #field: #binding, }
    });

    quote! {
        super::#module::#arguments_type { #(#bindings)* }: super::#module::#arguments_type
    }
}

fn arguments_module(
    function: &Ident,
    arguments: &[ConsoleArgument],
    slots: &[usize],
) -> Option<GeneratedModuleTokens> {
    if arguments.is_empty() {
        return None;
    }

    let arguments_type = bootstrap_arguments_type(function);
    let fields = arguments.iter().zip(slots).map(|(argument, slot)| {
        let field = console_argument_field_ident(*slot);
        let value_type = argument.field_type();

        quote! { pub #field: #value_type, }
    });

    Some(GeneratedModuleTokens::new(
        format!("container/build/{}", bootstrap_arguments_module(function)),
        quote! {
            pub struct #arguments_type {
                #(#fields)*
            }
        },
    ))
}

fn flow_statements(flow: &[&PlannedProvider]) -> Vec<TokenStream> {
    let mut weaver = ReverseConsoleArgumentWeaver::new();
    let mut statements: Vec<TokenStream> = flow
        .iter()
        .rev()
        .map(|planned| statement(planned, &mut weaver))
        .collect();

    statements.reverse();
    statements
}

fn root_builder(root: &PlannedProvider, plan: &ContainerPlan) -> RootBuilder {
    let flow = construction_flow(plan, &[root]);
    let provider = &root.provider;
    let function = format_ident!("construct_{}", provider.field_name);
    let parameters = arguments_declaration(&function, &root.console_slots);
    let statements = flow_statements(&flow);
    let root_binding = field_ident(provider);
    let root_type = field_type(provider);
    let error = construction_error_path();

    let body = quote! {
            #(#statements)*

            Ok(#root_binding)
    };

    let too_many_lines = too_many_lines_allow();
    let return_type = quote! { ::std::result::Result<#root_type, #error> };
    let errors_doc = construction_errors_doc();
    let tokens = if root.is_async {
        quote! {
            #errors_doc
            #too_many_lines
            pub async fn #function(#parameters) -> #return_type {
                #body
            }
        }
    } else {
        quote! {
            #errors_doc
            #too_many_lines
            pub fn #function(#parameters) -> #return_type {
                #body
            }
        }
    };

    RootBuilder {
        arguments: arguments_module(&function, &root.console_arguments, &root.console_slots),
        function,
        tokens,
    }
}

pub(crate) fn render_build(
    plan: &ContainerPlan,
    construction_roots: &[&PlannedProvider],
    retained_roots: &[&PlannedProvider],
    builder_roots: &[&PlannedProvider],
) -> Vec<GeneratedModuleTokens> {
    let root_builders = builder_roots
        .iter()
        .map(|root| root_builder(root, plan))
        .collect::<Vec<_>>();
    let serve_flow = construction_flow(plan, construction_roots);
    let ServeConsoleArguments {
        arguments: serve_arguments,
        slots: serve_slots,
    } = ServeConsoleArguments::from_roots(construction_roots);
    let serve_function = format_ident!("serve");
    let serve_parameters = arguments_declaration(&serve_function, &serve_slots);
    let serve_arguments_module = arguments_module(&serve_function, &serve_arguments, &serve_slots);
    let serve_statements = flow_statements(&serve_flow);
    let retained: std::collections::BTreeSet<_> =
        retained_roots.iter().map(|entry| &entry.key).collect();
    let fields = construction_roots
        .iter()
        .enumerate()
        .map(|(position, planned)| {
            let binding = field_ident(&planned.provider);
            let field = container_field_ident(position, retained.contains(&planned.key));

            quote! { #field: #binding, }
        })
        .collect::<Vec<_>>();
    let error = construction_error_path();

    let serve_return = quote! { ::std::result::Result<super::super::Container, #error> };
    let serve_body = quote! {
            #(#serve_statements)*

            Ok(super::super::Container {
                #(#fields)*
            })
    };
    let serve_too_many_lines = too_many_lines_allow();
    let serve_errors_doc = construction_errors_doc();
    let serve = if construction_roots.iter().any(|entry| entry.is_async) {
        quote! {
            #serve_errors_doc
            #serve_too_many_lines
            pub async fn serve(
                #serve_parameters
            ) -> #serve_return {
                #serve_body
            }
        }
    } else {
        quote! {
            #serve_errors_doc
            #serve_too_many_lines
            pub fn serve(
                #serve_parameters
            ) -> #serve_return {
                #serve_body
            }
        }
    };

    let builder_module_declarations = root_builders.iter().map(|builder| {
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
    let builder_module_exports = root_builders.iter().map(|builder| {
        let function = &builder.function;

        quote! { pub use #function::#function; }
    });
    let serve_arguments_declaration = serve_arguments_module
        .as_ref()
        .map(|_| bootstrap_arguments_module(&serve_function))
        .map(|module| quote! { pub mod #module; });
    let mut modules = vec![GeneratedModuleTokens::new(
        "container/build",
        quote! {
            #(#builder_module_declarations)*
            #serve_arguments_declaration
            mod serve;

            #(#builder_module_exports)*
            pub use serve::serve;
        },
    )];

    for builder in root_builders {
        modules.extend(builder.arguments);
        modules.push(GeneratedModuleTokens::new(
            format!("container/build/{}", builder.function),
            builder.tokens,
        ));
    }

    modules.extend(serve_arguments_module);
    modules.push(GeneratedModuleTokens::new("container/build/serve", serve));

    modules
}
