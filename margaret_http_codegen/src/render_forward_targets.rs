use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;

use crate::forward_targets_layout::ForwardTargetsLayout;
use crate::named_route::NamedRoute;
use crate::named_route_shape::NamedRouteShape;
use crate::route_props_layout::RoutePropsLayout;
use crate::server_forwarding::ServerForwarding;
use crate::server_layout::ServerLayout;

fn forward_target_of<'layout>(named: &'layout NamedRoute<'_>) -> Option<TargetedRoute<'layout>> {
    match &named.shape {
        NamedRouteShape::ForwardableLiteral {
            field,
            props,
            variant,
            ..
        }
        | NamedRouteShape::ForwardableParameterized {
            field,
            props,
            variant,
        } => Some(TargetedRoute {
            field,
            props,
            variant,
        }),
        NamedRouteShape::ReferenceLiteral { .. }
        | NamedRouteShape::ReferenceParameterized { .. } => None,
    }
}

fn target_enum(
    ForwardTargetsLayout {
        target_type,
        targets_type,
        ..
    }: &ForwardTargetsLayout,
    forwardable: &[TargetedRoute<'_>],
) -> TokenStream {
    let variants = forwardable.iter().map(
        |TargetedRoute {
             props: RoutePropsLayout { type_ident, .. },
             variant,
             ..
         }| quote! { #variant(super::super::props::#type_ident), },
    );
    let arms = forwardable.iter().map(
        |TargetedRoute { field, variant, .. }| {
            quote! {
                Self::#variant(props) => {
                    margaret::framework::http::forwarded_route::ForwardedRoute::new(&targets.#field, props)
                }
            }
        },
    );

    quote! {
        pub enum #target_type {
            #(#variants)*
        }

        impl margaret::framework::http::forward_target::ForwardTarget for #target_type {
            type Targets = super::#targets_type;

            fn resolve(
                self,
                targets: &Self::Targets,
            ) -> margaret::framework::http::forwarded_route::ForwardedRoute<Self> {
                match self {
                    #(#arms)*
                }
            }
        }
    }
}

fn targets_struct(
    ForwardTargetsLayout {
        target_type,
        targets_type,
        ..
    }: &ForwardTargetsLayout,
    forwardable: &[TargetedRoute<'_>],
) -> TokenStream {
    let fields = forwardable.iter().map(|TargetedRoute { field, .. }| {
        quote! {
            pub #field: ::std::sync::Arc<
                dyn margaret::framework::http::head_handler::HeadHandler<super::#target_type>,
            >,
        }
    });

    quote! {
        pub struct #targets_type {
            #(#fields)*
        }
    }
}

struct TargetedRoute<'layout> {
    field: &'layout Ident,
    props: &'layout RoutePropsLayout,
    variant: &'layout Ident,
}

struct ServerTargets<'layout> {
    forwardable: Vec<TargetedRoute<'layout>>,
    layout: &'layout ForwardTargetsLayout,
}

pub(crate) fn render_forward_targets(
    routes: &ModulePath,
    layouts: &[ServerLayout<'_>],
) -> Vec<GeneratedModuleTokens> {
    let forwarding: Vec<ServerTargets<'_>> = layouts
        .iter()
        .filter_map(|layout| match &layout.forwarding {
            ServerForwarding::Targets(targets) => Some(ServerTargets {
                forwardable: layout.named.iter().filter_map(forward_target_of).collect(),
                layout: targets,
            }),
            ServerForwarding::Unforwarded => None,
        })
        .collect();

    if forwarding.is_empty() {
        return Vec::new();
    }

    let declarations = forwarding.iter().map(
        |ServerTargets {
             layout:
                 ForwardTargetsLayout {
                     target_module,
                     target_type,
                     targets_module,
                     targets_type,
                 },
             ..
         }| {
            quote! {
                mod #target_module;
                mod #targets_module;
                pub use #target_module::#target_type;
                pub use #targets_module::#targets_type;
            }
        },
    );
    let forward_targets = routes.child("forward_targets");
    let mut generated = vec![GeneratedModuleTokens::new(
        forward_targets.clone(),
        quote! { #(#declarations)* },
    )];

    for ServerTargets {
        forwardable,
        layout,
    } in &forwarding
    {
        generated.push(GeneratedModuleTokens::new(
            forward_targets.child(layout.target_module.to_string()),
            target_enum(layout, forwardable),
        ));
        generated.push(GeneratedModuleTokens::new(
            forward_targets.child(layout.targets_module.to_string()),
            targets_struct(layout, forwardable),
        ));
    }

    generated
}
