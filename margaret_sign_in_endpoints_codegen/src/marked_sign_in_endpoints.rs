use margaret_attribute_arguments::attribute_arguments_reader::AttributeArgumentsReader;
use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_declaration_anchor::declaration_anchor::declaration_anchor;
use margaret_http_codegen::declared_routes::DeclaredRoutes;
use margaret_route_method::route_method::RouteMethod;

use crate::marked_callback::MarkedCallback;
use crate::marked_endpoint::MarkedEndpoint;
use crate::marked_start::MarkedStart;
use crate::sign_in_client::sign_in_client;
use crate::sign_in_endpoint_variant::SignInEndpointVariant;
use crate::sign_in_endpoints_codegen_error::SignInEndpointsCodegenError;
use crate::sign_in_endpoints_vocabulary::SIGN_IN_ENDPOINTS;

fn marked_endpoint(
    index: &AttributeIndex,
    anchor: &IndexedItem,
    declared_routes: &DeclaredRoutes,
    variant: SignInEndpointVariant,
    nested: &mut AttributeArgumentsReader,
) -> Result<MarkedEndpoint, SignInEndpointsCodegenError> {
    let route = anchor.canonical_path();

    match variant {
        SignInEndpointVariant::Callback => {
            let written = nested.take_path("landing_route")?.ok_or_else(|| {
                SignInEndpointsCodegenError::MissingLandingRoute {
                    anchor: route.to_string(),
                }
            })?;
            let landing = index.resolve_item_path(anchor, &written).ok_or_else(|| {
                SignInEndpointsCodegenError::UnknownLandingRoute {
                    anchor: route.to_string(),
                    written: format_path(&written),
                }
            })?;

            Ok(MarkedEndpoint::Callback {
                landing: declared_routes.redirect_target(&landing, route)?,
            })
        }
        SignInEndpointVariant::Start => Ok(MarkedEndpoint::Start),
    }
}

pub(crate) struct MarkedSignInEndpoints {
    pub(crate) callbacks: Vec<MarkedCallback>,
    pub(crate) starts: Vec<MarkedStart>,
}

impl MarkedSignInEndpoints {
    pub(crate) fn read(
        index: &AttributeIndex,
        declared_routes: &DeclaredRoutes,
    ) -> Result<Self, SignInEndpointsCodegenError> {
        let mut callbacks = Vec::new();
        let mut starts = Vec::new();

        for matched in index.select_framework_attribute(FrameworkAttribute::ServesSignIn) {
            let anchor =
                declaration_anchor(index, &matched, FrameworkAttribute::ServesSignIn)?.item;
            let route = anchor.canonical_path();
            let Some(declared) = declared_routes
                .routes
                .iter()
                .find(|declared| declared.item.canonical_path() == route)
            else {
                return Err(SignInEndpointsCodegenError::UnroutedSignInEndpoint {
                    anchor: route.to_string(),
                });
            };

            if declared.method != RouteMethod::Get {
                return Err(SignInEndpointsCodegenError::SignInEndpointMethod {
                    anchor: route.to_string(),
                    method: declared.method,
                });
            }

            matched.args()?.interpret(|reader| {
                let endpoint = reader
                    .take_positional_variant(|written, nested| {
                        let variant = index
                            .resolve_item_path(anchor, written)
                            .as_ref()
                            .and_then(|resolved| SIGN_IN_ENDPOINTS.variant(resolved))
                            .ok_or_else(|| SignInEndpointsCodegenError::UnknownSignInEndpoint {
                                anchor: route.to_string(),
                                written: format_path(written),
                            })?;

                        marked_endpoint(index, anchor, declared_routes, variant, nested)
                    })?
                    .ok_or_else(|| SignInEndpointsCodegenError::MissingSignInEndpoint {
                        anchor: route.to_string(),
                    })?;
                let client = sign_in_client(reader, route)?;

                match endpoint {
                    MarkedEndpoint::Callback { landing } => callbacks.push(MarkedCallback {
                        client,
                        landing,
                        route: route.clone(),
                    }),
                    MarkedEndpoint::Start => starts.push(MarkedStart {
                        client,
                        route: route.clone(),
                    }),
                }

                Ok::<(), SignInEndpointsCodegenError>(())
            })?;
        }

        Ok(Self { callbacks, starts })
    }
}
