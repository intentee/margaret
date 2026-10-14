use proc_macro2::Literal;
use proc_macro2::TokenStream;
use quote::quote;

use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_generated_module::module_path::ModulePath;

use crate::named_route::NamedRoute;
use crate::named_route_shape::NamedRouteShape;
use crate::props_segment::PropsSegment;
use crate::route_props_layout::RoutePropsLayout;
use crate::server_layout::ServerLayout;

fn segment_tokens(segment: &PropsSegment) -> TokenStream {
    match segment {
        PropsSegment::CatchAllParameter {
            field,
            name,
            prefix,
        } => quote! {
            margaret::framework::http::url_segment::UrlSegment::CatchAllParameter {
                parameter: margaret::framework::http::url_parameter::UrlParameter {
                    name: #name,
                    value: self.#field,
                },
                prefix: #prefix,
            }
        },
        PropsSegment::Literal { decoded, encoded } => quote! {
            margaret::framework::http::url_segment::UrlSegment::Literal {
                decoded: #decoded,
                encoded: #encoded,
            }
        },
        PropsSegment::Parameter {
            field,
            name,
            prefix,
        } => quote! {
            margaret::framework::http::url_segment::UrlSegment::Parameter {
                parameter: margaret::framework::http::url_parameter::UrlParameter {
                    name: #name,
                    value: self.#field,
                },
                prefix: #prefix,
            }
        },
    }
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
            PropsSegment::CatchAllParameter { field, .. }
            | PropsSegment::Parameter { field, .. } => {
                Some(quote! { pub #field: ::std::string::String, })
            }
            PropsSegment::Literal { .. } => None,
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

    let modules = props
        .iter()
        .map(|NamedRouteProps { props, .. }| &props.module);
    let exports = props.iter().map(
        |NamedRouteProps {
             props: RoutePropsLayout {
                 module, type_ident, ..
             },
             ..
         }| quote! { pub use #module::#type_ident; },
    );
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

    generated
}
