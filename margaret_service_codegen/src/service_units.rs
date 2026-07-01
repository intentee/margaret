use syn::FnArg;
use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::type_leaf_ident::type_leaf_ident;

use crate::service_codegen_error::ServiceCodegenError;
use crate::service_kind::ServiceKind;
use crate::service_unit::ServiceUnit;
use crate::tick_timer_arguments::TickTimerArguments;

#[derive(Clone, Copy)]
enum Role {
    Service,
    Ticker,
}

fn build_unit(matched: &MatchedAttribute, role: Role) -> Result<ServiceUnit, ServiceCodegenError> {
    let item = matched.item();
    let path = item.canonical_path().to_string();

    if !item.kind().is_struct() {
        return Err(match role {
            Role::Service => ServiceCodegenError::ServiceNotAStruct { path },
            Role::Ticker => ServiceCodegenError::TickerNotAStruct { path },
        });
    }

    if has_conflicting_roles(item, role) {
        return Err(ServiceCodegenError::ConflictingRoles { path });
    }

    let runner = resolve_runner(item, &path)?;
    let takes_token = runner_takes_token(runner, &path)?;
    let kind = match role {
        Role::Service => ServiceKind::Service,
        Role::Ticker => {
            let TickTimerArguments { behavior, interval } =
                TickTimerArguments::parse(&matched.args()?, &path)?;

            ServiceKind::Ticker { behavior, interval }
        }
    };

    Ok(ServiceUnit {
        concrete_path: item.canonical_path().clone(),
        field_name: item.canonical_path().field_name(),
        kind,
        runner: runner.identifier().to_string(),
        takes_token,
    })
}

fn has_conflicting_roles(item: &IndexedItem, role: Role) -> bool {
    let others = match role {
        Role::Service => ["console_command", "scheduled_with_tick_timer"],
        Role::Ticker => ["console_command", "service"],
    };

    others.iter().any(|other| {
        let selector = selector(other);

        item.attributes()
            .iter()
            .any(|attribute| selector.matches(attribute.path()))
    })
}

fn resolve_runner<'item>(
    item: &'item IndexedItem,
    path: &str,
) -> Result<&'item IndexedMethod, ServiceCodegenError> {
    let selector = selector("runner");
    let mut found: Vec<&IndexedMethod> = item
        .methods()
        .iter()
        .filter(|method| {
            method
                .attributes()
                .iter()
                .any(|attribute| selector.matches(attribute.path()))
        })
        .collect();

    if found.len() > 1 {
        return Err(ServiceCodegenError::AmbiguousRunner {
            unit: path.to_string(),
            methods: found
                .iter()
                .map(|method| method.identifier().to_string())
                .collect::<Vec<String>>()
                .join(", "),
        });
    }

    found
        .pop()
        .ok_or_else(|| ServiceCodegenError::MissingRunner {
            unit: path.to_string(),
        })
}

fn runner_takes_token(method: &IndexedMethod, path: &str) -> Result<bool, ServiceCodegenError> {
    let mut takes_token = false;

    for (position, input) in method.signature().inputs.iter().enumerate() {
        match input {
            FnArg::Receiver(_) => {}
            FnArg::Typed(pattern_type) if is_cancellation_token(&pattern_type.ty) => {
                takes_token = true;
            }
            FnArg::Typed(_) => {
                return Err(ServiceCodegenError::UnexpectedRunnerParameter {
                    unit: path.to_string(),
                    parameter: position.to_string(),
                });
            }
        }
    }

    Ok(takes_token)
}

fn is_cancellation_token(declared: &Type) -> bool {
    type_leaf_ident(declared).is_some_and(|ident| ident == "CancellationToken")
}

fn selector(name: &str) -> AttributeSelector {
    AttributeSelector::parse(name).expect("a marker selector is valid")
}

pub(crate) fn service_units(
    index: &AttributeIndex,
) -> Result<Vec<ServiceUnit>, ServiceCodegenError> {
    let mut units = Vec::new();

    for matched in index.select(&selector("service")) {
        units.push(build_unit(&matched, Role::Service)?);
    }

    for matched in index.select(&selector("scheduled_with_tick_timer")) {
        units.push(build_unit(&matched, Role::Ticker)?);
    }

    Ok(units)
}
