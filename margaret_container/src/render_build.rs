use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::struct_shape::StructShape;
use margaret_codegen_tokens::console_argument_ident::console_argument_ident;
use margaret_codegen_tokens::path_tokens::path_tokens;
use margaret_console_argument_codegen::console_argument::ConsoleArgument;

use crate::console_closures::ConsoleClosures;
use crate::construction_error_path::construction_error_path;
use crate::construction_flow::construction_flow;
use crate::container_plan::ContainerPlan;
use crate::dependency_kind::DependencyKind;
use crate::direct_construction::DirectConstruction;
use crate::field_ident::field_ident;
use crate::provider::Provider;
use crate::render::field_type;
use crate::reverse_console_argument_weaver::ReverseConsoleArgumentWeaver;

fn fieldless_literal(concrete: &TokenStream, shape: StructShape) -> TokenStream {
    match shape {
        StructShape::Named { .. } => quote! { #concrete {} },
        StructShape::Unit => quote! { #concrete },
        StructShape::Unnamed { .. } => quote! { #concrete() },
    }
}

fn dependency_expression(
    dependency: &DependencyKind,
    closures: &ConsoleClosures,
    weaver: &mut ReverseConsoleArgumentWeaver,
    plan: &ContainerPlan,
) -> TokenStream {
    match dependency {
        DependencyKind::ConsoleArgument { argument } => {
            weaver.weave(argument, closures.slot(&argument.slot_key()))
        }
        DependencyKind::Single { provider_key } => {
            let dependency = field_ident(plan.entry(provider_key));

            quote! { ::std::sync::Arc::clone(&#dependency) }
        }
    }
}

fn dependency_expressions(
    dependencies: &[DependencyKind],
    closures: &ConsoleClosures,
    weaver: &mut ReverseConsoleArgumentWeaver,
    plan: &ContainerPlan,
) -> Vec<TokenStream> {
    let mut expressions: Vec<TokenStream> = dependencies
        .iter()
        .rev()
        .map(|dependency| dependency_expression(dependency, closures, weaver, plan))
        .collect();

    expressions.reverse();
    expressions
}

fn direct_value(
    provider: &Provider,
    closures: &ConsoleClosures,
    weaver: &mut ReverseConsoleArgumentWeaver,
    plan: &ContainerPlan,
) -> TokenStream {
    let concrete = path_tokens(&provider.concrete_path);

    match &provider.construction {
        DirectConstruction::Constructor {
            dependencies,
            is_async,
            method,
        } => {
            let constructor = format_ident!("{method}");
            let arguments = dependency_expressions(dependencies, closures, weaver, plan);
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
            dependencies,
            is_async,
            method,
        } => {
            let constructor = format_ident!("{method}");
            let arguments = dependency_expressions(dependencies, closures, weaver, plan);

            if *is_async {
                quote! { #concrete::#constructor(#(#arguments),*).await }
            } else {
                quote! { #concrete::#constructor(#(#arguments),*) }
            }
        }
        DirectConstruction::FrameworkUnit => quote! { #concrete },
        DirectConstruction::FrameworkAccessor {
            accessor,
            dependencies,
        } => {
            let accessor = format_ident!("{accessor}");
            let sources = dependency_expressions(dependencies, closures, weaver, plan);

            quote! { #(#sources)*.#accessor() }
        }
        DirectConstruction::Resolved {
            dependencies,
            resolver,
        } => {
            let resolver = path_tokens(resolver);
            let arguments = dependency_expressions(dependencies, closures, weaver, plan);

            quote! { #resolver(#(#arguments),*) }
        }
    }
}

fn statement(
    key: &CanonicalPath,
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
    weaver: &mut ReverseConsoleArgumentWeaver,
) -> TokenStream {
    let provider = plan.entry(key);
    let binding = field_ident(provider);
    let value_type = field_type(provider);
    let value = direct_value(provider, closures, weaver, plan);
    let constructed = match &provider.construction {
        DirectConstruction::FrameworkAccessor { .. } | DirectConstruction::Resolved { .. } => value,
        DirectConstruction::Constructor { .. }
        | DirectConstruction::Fieldless { .. }
        | DirectConstruction::FrameworkConstructor { .. }
        | DirectConstruction::FrameworkUnit => quote! { ::std::sync::Arc::new(#value) },
    };

    quote! { let #binding: #value_type = #constructed; }
}

fn parameters(arguments: &[ConsoleArgument], closures: &ConsoleClosures) -> Vec<TokenStream> {
    arguments
        .iter()
        .map(|argument| {
            let binding = console_argument_ident(closures.slot(&argument.slot_key()));
            let value_type = argument.field_type();

            quote! { #binding: #value_type }
        })
        .collect()
}

fn flow_statements(
    flow: &[CanonicalPath],
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
) -> Vec<TokenStream> {
    let mut weaver = ReverseConsoleArgumentWeaver::new();
    let mut statements: Vec<TokenStream> = flow
        .iter()
        .rev()
        .map(|key| statement(key, plan, closures, &mut weaver))
        .collect();

    statements.reverse();
    statements
}

fn root_builder(
    root: &CanonicalPath,
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
) -> TokenStream {
    let flow = construction_flow(plan, std::slice::from_ref(root));
    let provider = plan.entry(root);
    let function = format_ident!("construct_{}", provider.field_name);
    let arguments = closures.of(root);
    let parameters = parameters(arguments, closures);
    let statements = flow_statements(&flow, plan, closures);
    let root_binding = field_ident(provider);
    let root_type = field_type(provider);
    let error = construction_error_path();

    quote! {
        pub async fn #function(#(#parameters),*) -> ::std::result::Result<#root_type, #error> {
            #(#statements)*

            Ok(#root_binding)
        }
    }
}

fn serve_arguments(roots: &[CanonicalPath], closures: &ConsoleClosures) -> Vec<ConsoleArgument> {
    let slots: std::collections::BTreeSet<usize> = roots
        .iter()
        .flat_map(|root| closures.of(root))
        .map(|argument| closures.slot(&argument.slot_key()))
        .collect();

    slots
        .into_iter()
        .map(|slot| closures.argument(slot).clone())
        .collect()
}

pub(crate) fn render_build(
    plan: &ContainerPlan,
    closures: &ConsoleClosures,
    serve_roots: &[CanonicalPath],
) -> TokenStream {
    let root_builders = plan
        .dependency_order
        .iter()
        .map(|root| root_builder(root, plan, closures))
        .collect::<Vec<_>>();
    let serve_flow = construction_flow(plan, serve_roots);
    let serve_arguments = serve_arguments(serve_roots, closures);
    let serve_parameters = parameters(&serve_arguments, closures);
    let serve_statements = flow_statements(&serve_flow, plan, closures);
    let fields = serve_roots.iter().map(|key| {
        let field = field_ident(plan.entry(key));

        quote! { #field, }
    });
    let error = construction_error_path();

    quote! {
        #(#root_builders)*

        pub async fn serve(
            #(#serve_parameters),*
        ) -> ::std::result::Result<super::Container, #error> {
            #(#serve_statements)*

            Ok(super::Container {
                #(#fields)*
            })
        }
    }
}
