use quote::format_ident;
use syn::Expr;
use syn::ExprLit;
use syn::Lit;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;

use crate::http_codegen_error::HttpCodegenError;
use crate::middleware_binding::MiddlewareBinding;

pub(crate) fn middleware_bindings(
    index: &AttributeIndex,
) -> Result<Vec<MiddlewareBinding>, HttpCodegenError> {
    let selector = AttributeSelector::parse("http_middleware").expect("a valid selector");
    let mut bindings = Vec::new();

    for matched in index.select(&selector) {
        let item = matched.item();

        if !item.kind().is_struct() {
            return Err(HttpCodegenError::HttpMiddlewareNotOnStruct {
                target: item.canonical_path().to_string(),
            });
        }

        let arguments = matched.args()?;
        let middleware = item.canonical_path().to_string();
        let handles =
            arguments
                .path("handles")?
                .ok_or(HttpCodegenError::MissingMiddlewareHandles {
                    middleware: middleware.clone(),
                })?;
        let priority = match arguments.named("priority") {
            Some(Expr::Lit(ExprLit {
                lit: Lit::Int(value),
                ..
            })) => value
                .base10_parse::<i64>()
                .expect("a priority that fits in i64"),
            Some(_) => return Err(HttpCodegenError::MalformedMiddlewarePriority { middleware }),
            None => return Err(HttpCodegenError::MissingMiddlewarePriority { middleware }),
        };

        bindings.push(MiddlewareBinding {
            field: format_ident!("{}", item.canonical_path().field_name()),
            priority,
            selector: AttributeSelector::from_path(handles),
        });
    }

    Ok(bindings)
}
