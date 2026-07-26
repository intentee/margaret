use margaret_generated_module::generated_module_tokens::GeneratedModuleTokens;
use margaret_request_binding_codegen::authenticated_user_login_route::AuthenticatedUserLoginRoute;
use margaret_request_binding_codegen::authenticated_user_provider::AuthenticatedUserProvider;
use margaret_request_binding_codegen::authenticated_user_wrapper_binding::AuthenticatedUserWrapperBinding;
use margaret_request_binding_codegen::binding_registries::BindingRegistries;
use margaret_request_binding_codegen::render_authenticated_user_wrappers::render_authenticated_user_wrappers;
use margaret_request_binding_codegen::request_binding::RequestBinding;

use crate::http_codegen_error::HttpCodegenError;
use crate::http_route_table::HttpRouteTable;
use crate::responder_route::ResponderRoute;

fn resolve_login_route(
    table: &HttpRouteTable,
    provider: &AuthenticatedUserProvider,
) -> Result<AuthenticatedUserLoginRoute, HttpCodegenError> {
    let provider_name = provider.application.concrete.to_string();
    let login_route = provider.application.login_route.to_string();

    let Some(ResponderRoute { path, route }) =
        table.find_by_responder(&provider.application.login_route)
    else {
        return Err(HttpCodegenError::AuthenticatedUserLoginRouteNotARoute {
            provider: provider_name,
            login_route,
        });
    };

    if route.method != "GET" {
        return Err(HttpCodegenError::AuthenticatedUserLoginRouteNotGet {
            provider: provider_name,
            login_route,
        });
    }

    let Some(name) = &route.name else {
        return Err(HttpCodegenError::AuthenticatedUserLoginRouteUnnamed {
            provider: provider_name,
            login_route,
        });
    };

    if path.parameters().next().is_some() {
        return Err(HttpCodegenError::AuthenticatedUserLoginRouteParameterized {
            provider: provider_name,
            login_route,
        });
    }

    if route
        .arguments
        .iter()
        .any(|argument| matches!(argument.binding, RequestBinding::AuthenticatedUser { .. }))
    {
        return Err(
            HttpCodegenError::AuthenticatedUserLoginRouteRequiresAuthentication {
                provider: provider_name,
                login_route,
            },
        );
    }

    Ok(AuthenticatedUserLoginRoute {
        route: name.clone(),
        server: route.server.clone(),
    })
}

pub(crate) fn render_authenticated_users(
    table: &HttpRouteTable,
    registries: &BindingRegistries,
) -> Result<Vec<GeneratedModuleTokens>, HttpCodegenError> {
    let providers = registries.providers();

    if providers.is_empty() {
        return Ok(Vec::new());
    }

    let mut bindings = Vec::with_capacity(providers.len());

    for provider in providers {
        let login_route = resolve_login_route(table, provider)?;

        bindings.push(AuthenticatedUserWrapperBinding {
            login_route,
            provider,
        });
    }

    Ok(vec![GeneratedModuleTokens::new(
        "authenticated_users",
        render_authenticated_user_wrappers(&bindings),
    )])
}
