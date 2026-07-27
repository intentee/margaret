use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_injection_codegen::is_cancellation_token::is_cancellation_token;
use margaret_injection_codegen::parameters::parameters;
use margaret_injection_codegen::process_method::process_method;
use margaret_injection_codegen::request_binding_marker::request_binding_marker;

use crate::service_codegen_error::ServiceCodegenError;
use crate::service_kind::ServiceKind;
use crate::service_unit::ServiceUnit;
use crate::service_unit_origin::ServiceUnitOrigin;
use crate::tick_timer_arguments::TickTimerArguments;

#[derive(Clone, Copy)]
enum Role {
    Service,
    Ticker,
}

fn validate_runner(
    index: &AttributeIndex,
    item: &IndexedItem,
    runner: &IndexedMethod,
    path: &str,
) -> Result<(), ServiceCodegenError> {
    for view in parameters(runner.signature()) {
        if let Some(name) = request_binding_marker(view.attributes) {
            return Err(ServiceCodegenError::RunnerRequestBinding {
                path: path.to_string(),
                parameter: view.holder.to_string(),
                marker: name.to_string(),
            });
        }

        if !is_cancellation_token(index, item, view.declared) {
            return Err(ServiceCodegenError::RunnerArgument {
                path: path.to_string(),
                parameter: view.holder.to_string(),
            });
        }
    }

    Ok(())
}

fn build_unit(
    matched: &MatchedAttribute,
    index: &AttributeIndex,
    role: Role,
) -> Result<ServiceUnit, ServiceCodegenError> {
    let item = matched.item();
    let path = item.canonical_path().to_string();

    let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
        return Err(match role {
            Role::Service => ServiceCodegenError::ServiceNotAStruct { path },
            Role::Ticker => ServiceCodegenError::TickerNotAStruct { path },
        });
    };

    if has_conflicting_roles(item, role) {
        return Err(ServiceCodegenError::ConflictingRoles { path });
    }

    let runner = process_method(item)?;

    validate_runner(index, item, runner, &path)?;

    let takes_token = runner_takes_token(index, item, runner);
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
        field_name: identifier.field().to_string(),
        kind,
        origin: ServiceUnitOrigin::User,
        runner: runner.identifier().to_string(),
        takes_token,
        type_name: identifier.type_name().to_string(),
    })
}

fn has_conflicting_roles(item: &IndexedItem, role: Role) -> bool {
    let others = match role {
        Role::Service => ["console_command", "scheduled_with_tick_timer"],
        Role::Ticker => ["console_command", "service"],
    };

    others
        .iter()
        .any(|other| item.has_attribute(&selector(other)))
}

fn runner_takes_token(index: &AttributeIndex, item: &IndexedItem, method: &IndexedMethod) -> bool {
    parameters(method.signature())
        .iter()
        .any(|view| is_cancellation_token(index, item, view.declared))
}

fn selector(name: &str) -> AttributeSelector {
    AttributeSelector::from_marker(name)
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
