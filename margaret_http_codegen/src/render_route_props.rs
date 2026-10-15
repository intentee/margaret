use proc_macro2::Ident;
use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;

use crate::named_route::NamedRoute;
use crate::named_route_shape::NamedRouteShape;
use crate::props_prefix::PropsPrefix;
use crate::props_segment::PropsSegment;
use crate::route_props_layout::RoutePropsLayout;
use crate::server_layout::ServerLayout;

fn prefix_type(prefix: &PropsPrefix) -> TokenStream {
    match prefix {
        PropsPrefix::Prefixed { type_ident, .. } => quote! { super::#type_ident },
        PropsPrefix::Unprefixed => {
            quote! { margaret::framework::url_path::unprefixed_segment::UnprefixedSegment }
        }
    }
}

fn segment_tokens(segment: &PropsSegment) -> TokenStream {
    match segment {
        PropsSegment::CatchAllParameter { field, name, .. } => quote! {
            margaret::framework::http::url_segment::UrlSegment::catch_all(#name, self.#field)
        },
        PropsSegment::Literal { decoded, encoded } => quote! {
            margaret::framework::http::url_segment::UrlSegment::Literal {
                decoded: #decoded,
                encoded: #encoded,
            }
        },
        PropsSegment::Parameter { field, name, .. } => quote! {
            margaret::framework::http::url_segment::UrlSegment::parameter(#name, self.#field)
        },
    }
}

fn prefix_module(prefix: &PropsPrefix) -> Option<PrefixModule<'_>> {
    match prefix {
        PropsPrefix::Prefixed {
            module,
            text,
            type_ident,
        } => Some(PrefixModule {
            module,
            text,
            type_ident,
        }),
        PropsPrefix::Unprefixed => None,
    }
}

fn prefix_modules(props: &RoutePropsLayout) -> impl Iterator<Item = PrefixModule<'_>> {
    props.segments.iter().filter_map(|segment| match segment {
        PropsSegment::CatchAllParameter { prefix, .. } | PropsSegment::Parameter { prefix, .. } => {
            prefix_module(prefix)
        }
        PropsSegment::Literal { .. } => None,
    })
}

fn props_struct(name: &str, props: &RoutePropsLayout) -> TokenStream {
    let RoutePropsLayout {
        segments,
        type_ident,
        ..
    } = props;
    let fields: Vec<TokenStream> = segments
        .iter()
        .filter_map(|segment| match segment {
            PropsSegment::CatchAllParameter { field, prefix, .. } => {
                let prefix = prefix_type(prefix);

                Some(quote! {
                    pub #field: margaret::framework::url_path::path_tail_value::PathTailValue<#prefix>,
                })
            }
            PropsSegment::Literal { .. } => None,
            PropsSegment::Parameter { field, prefix, .. } => {
                let prefix = prefix_type(prefix);

                Some(quote! {
                    pub #field: margaret::framework::url_path::path_parameter_value::PathParameterValue<#prefix>,
                })
            }
        })
        .collect();
    let declaration = if fields.is_empty() {
        quote! { pub struct #type_ident; }
    } else {
        quote! {
            pub struct #type_ident {
                #(#fields)*
            }
        }
    };
    let segment_count = Literal::usize_unsuffixed(segments.len());
    let segments = segments.iter().map(segment_tokens);

    quote! {
        #declaration

        impl margaret::framework::http::route_props::RouteProps for #type_ident {
            const NAME: &'static str = #name;

            type Segments = [margaret::framework::http::url_segment::UrlSegment; #segment_count];

            fn into_segments(self) -> Self::Segments {
                [#(#segments),*]
            }
        }
    }
}

fn route_props<'layout>(named: &'layout NamedRoute<'_>) -> Option<&'layout RoutePropsLayout> {
    match &named.shape {
        NamedRouteShape::ForwardableLiteral { props, .. }
        | NamedRouteShape::ForwardableParameterized { props, .. }
        | NamedRouteShape::ReferenceParameterized { props } => Some(props),
        NamedRouteShape::ReferenceLiteral { .. } => None,
    }
}

struct NamedRouteProps<'layout> {
    name: &'layout str,
    props: &'layout RoutePropsLayout,
}

struct PrefixModule<'layout> {
    module: &'layout Ident,
    text: &'layout str,
    type_ident: &'layout Ident,
}

pub(crate) fn render_route_props(
    routes: &ModulePath,
    layouts: &[ServerLayout<'_>],
) -> Vec<GeneratedModuleTokens> {
    let props: Vec<NamedRouteProps<'_>> = layouts
        .iter()
        .flat_map(|layout| &layout.named)
        .filter_map(|named| {
            route_props(named).map(|props| NamedRouteProps {
                name: named.name,
                props,
            })
        })
        .collect();

    if props.is_empty() {
        return Vec::new();
    }

    let prefixes: Vec<PrefixModule<'_>> = props
        .iter()
        .flat_map(|NamedRouteProps { props, .. }| prefix_modules(props))
        .collect();
    let modules = props
        .iter()
        .map(|NamedRouteProps { props, .. }| &props.module)
        .chain(prefixes.iter().map(|prefix| prefix.module));
    let exports = props
        .iter()
        .map(
            |NamedRouteProps {
                 props:
                     RoutePropsLayout {
                         module, type_ident, ..
                     },
                 ..
             }| quote! { pub use #module::#type_ident; },
        )
        .chain(prefixes.iter().map(
            |PrefixModule {
                 module, type_ident, ..
             }| quote! { pub use #module::#type_ident; },
        ));
    let props_module = routes.child("props");
    let mut generated = vec![GeneratedModuleTokens::new(
        props_module.clone(),
        quote! {
            #(mod #modules;)*
            #(#exports)*
        },
    )];

    generated.extend(props.iter().map(|NamedRouteProps { name, props }| {
        GeneratedModuleTokens::new(
            props_module.child(props.module.to_string()),
            props_struct(name, props),
        )
    }));
    generated.extend(prefixes.iter().map(
        |PrefixModule {
             module,
             text,
             type_ident,
         }| {
            GeneratedModuleTokens::new(
                props_module.child(module.to_string()),
                quote! {
                    pub struct #type_ident;

                    impl margaret::framework::url_path::segment_prefix::SegmentPrefix for #type_ident {
                        const TEXT: &'static str = #text;
                    }
                },
            )
        },
    ));

    generated
}
