use margaret_attributes::attribute_holder::AttributeHolder;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::item_kind::ItemKind;
use proc_macro2::Ident;
use quote::format_ident;
use syn::Expr;
use syn::ExprLit;
use syn::Lit;

use crate::field_name::field_name;
use crate::http_codegen_error::HttpCodegenError;

pub(crate) struct MiddlewareBinding {
    pub(crate) field: Ident,
    pub(crate) priority: i64,
    pub(crate) selector: AttributeSelector,
}

pub(crate) fn middleware_bindings(
    index: &AttributeIndex,
) -> Result<Vec<MiddlewareBinding>, HttpCodegenError> {
    let selector = AttributeSelector::parse("http_middleware").expect("a valid selector");
    let mut bindings = Vec::new();

    for matched in index.select(&selector) {
        let item = match matched.holder() {
            AttributeHolder::Item(item) if item.kind() == ItemKind::Struct => item,
            holder => {
                return Err(HttpCodegenError::HttpMiddlewareNotOnStruct {
                    target: holder.target_path(),
                });
            }
        };

        let arguments = matched.args()?;
        let middleware = item.canonical_path().to_string();
        let handles = arguments
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
            field: format_ident!("{}", field_name(item.canonical_path())),
            priority,
            selector: AttributeSelector::from_path(handles),
        });
    }

    Ok(bindings)
}
