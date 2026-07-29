use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::struct_shape::StructShape;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;
use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;

use crate::construction_error_path::construction_error_path;
use crate::construction_flow::construction_flow;
use crate::container_field_ident::container_field_ident;
use crate::container_plan::ContainerPlan;
use crate::direct_construction::DirectConstruction;
use crate::field_ident::field_ident;
use crate::field_type::field_type;
use crate::planned_dependency::PlannedDependency;
use crate::planned_provider::PlannedProvider;
use crate::reverse_console_argument_weaver::ReverseConsoleArgumentWeaver;

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
            let construction_error = construction_error_path();
            let singleton = provider.concrete_path.to_string();
            let call = if *is_async {
                quote! { #concrete::#constructor(#(#arguments),*).await }
            } else {
                quote! { #concrete::#constructor(#(#arguments),*) }
            };

            quote! { #construction_error::wrap(#singleton, #call)? }
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
    let value_type = field_type(provider);
    let value = direct_value(planned, weaver);
    let constructed = match &provider.construction {
        DirectConstruction::FrameworkAccessor { .. } | DirectConstruction::Resolved { .. } => value,
        DirectConstruction::Constructor { .. }
        | DirectConstruction::Fieldless { .. }
        | DirectConstruction::FrameworkConstructor { .. }
        | DirectConstruction::FrameworkUnit => quote! { ::std::sync::Arc::new(#value) },
    };

    quote! { let #binding: #value_type = #constructed; }
}

fn parameters(arguments: &[ConsoleArgument], slots: &[usize]) -> Vec<TokenStream> {
    arguments
        .iter()
        .zip(slots)
        .map(|(argument, slot)| {
            let binding = console_argument_ident(*slot);
            let value_type = argument.field_type();

            quote! { #binding: #value_type }
        })
        .collect()
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

fn root_builder(root: &PlannedProvider, plan: &ContainerPlan) -> (proc_macro2::Ident, TokenStream) {
    let flow = construction_flow(plan, &[root]);
    let provider = &root.provider;
    let function = format_ident!("construct_{}", provider.field_name);
    let parameters = parameters(&root.console_arguments, &root.console_slots);
    let statements = flow_statements(&flow);
    let root_binding = field_ident(provider);
    let root_type = field_type(provider);
    let error = construction_error_path();

    let body = quote! {
            #(#statements)*

            Ok(#root_binding)
    };

    let tokens = if root.is_async {
        quote! {
            pub(crate) async fn #function(#(#parameters),*) -> ::std::result::Result<#root_type, #error> {
                #body
            }
        }
    } else {
        quote! {
            pub(crate) fn #function(#(#parameters),*) -> ::std::result::Result<#root_type, #error> {
                #body
            }
        }
    };

    (function, tokens)
}

fn serve_arguments(roots: &[&PlannedProvider]) -> (Vec<ConsoleArgument>, Vec<usize>) {
    let mut arguments_by_slot = std::collections::BTreeMap::new();

    for root in roots {
        for (argument, slot) in root
            .console_arguments
            .iter()
            .zip(root.console_slots.iter().copied())
        {
            arguments_by_slot.entry(slot).or_insert(argument);
        }
    }

    arguments_by_slot
        .into_iter()
        .map(|(slot, argument)| (argument.clone(), slot))
        .unzip()
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
    let (serve_arguments, serve_slots) = serve_arguments(construction_roots);
    let serve_parameters = parameters(&serve_arguments, &serve_slots);
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

    let serve_body = quote! {
            #(#serve_statements)*

            Ok(super::super::Container {
                #(#fields)*
            })
    };
    let serve = if construction_roots.iter().any(|entry| entry.is_async) {
        quote! {
            pub(crate) async fn serve(
                #(#serve_parameters),*
            ) -> ::std::result::Result<super::super::Container, #error> {
                #serve_body
            }
        }
    } else {
        quote! {
            pub(crate) fn serve(
            #(#serve_parameters),*
        ) -> ::std::result::Result<super::super::Container, #error> {
                #serve_body
            }
        }
    };

    let builder_modules = root_builders.iter().map(|(function, _)| {
        quote! {
            pub(crate) mod #function;
            pub(crate) use #function::#function;
        }
    });
    let mut modules = vec![GeneratedModuleTokens::new(
        "container/build",
        quote! {
            #(#builder_modules)*
            pub(crate) mod serve;
            pub(crate) use serve::serve;
        },
    )];

    modules.extend(root_builders.into_iter().map(|(function, tokens)| {
        GeneratedModuleTokens::new(format!("container/build/{function}"), tokens)
    }));
    modules.push(GeneratedModuleTokens::new("container/build/serve", serve));

    modules
}
