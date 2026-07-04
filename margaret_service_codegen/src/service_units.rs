use syn::Type;

use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_attributes::type_leaf_ident::type_leaf_ident;
use margaret_injection_codegen::parameter_view::ParameterView;
use margaret_injection_codegen::parameters::parameters;
use margaret_injection_codegen::process_method::process_method;

use crate::service_codegen_error::ServiceCodegenError;
use crate::service_kind::ServiceKind;
use crate::service_unit::ServiceUnit;
use crate::tick_timer_arguments::TickTimerArguments;

#[derive(Clone, Copy)]
enum Role {
    Service,
    Ticker,
}

fn build_unit(
    matched: &MatchedAttribute,
    index: &AttributeIndex,
    role: Role,
) -> Result<ServiceUnit, ServiceCodegenError> {
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

    let runner = process_method(item)?;
    let takes_token = runner_takes_token(runner, &path)?;
    let kind = match role {
        Role::Service => ServiceKind::Service,
        Role::Ticker => {
            let TickTimerArguments { behavior, interval } =
                TickTimerArguments::parse(matched.args()?, &path)?;

            ServiceKind::Ticker { behavior, interval }
        }
    };

    Ok(ServiceUnit {
        concrete_path: item.canonical_path().clone(),
        field_name: index.field_name(item.canonical_path()).to_string(),
        kind,
        runner: runner.identifier().to_string(),
        takes_token,
        type_name: index.type_name(item.canonical_path()).to_string(),
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

fn runner_takes_token(method: &IndexedMethod, path: &str) -> Result<bool, ServiceCodegenError> {
    let mut takes_token = false;

    for ParameterView {
        declared, position, ..
    } in parameters(method.signature())
    {
        if is_cancellation_token(declared) {
            takes_token = true;
        } else {
            return Err(ServiceCodegenError::UnexpectedProcessParameter {
                unit: path.to_string(),
                parameter: position.to_string(),
            });
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
        units.push(build_unit(&matched, index, Role::Service)?);
    }

    for matched in index.select(&selector("scheduled_with_tick_timer")) {
        units.push(build_unit(&matched, index, Role::Ticker)?);
    }

    Ok(units)
}
