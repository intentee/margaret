use syn::Path;

use margaret_attribute_arguments::format_path::format_path;
use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::indexed_item::IndexedItem;
use margaret_attributes::indexed_method::IndexedMethod;
use margaret_attributes::matched_attribute::MatchedAttribute;
use margaret_injection_codegen::is_cancellation_token::is_cancellation_token;
use margaret_injection_codegen::parameters::parameters;
use margaret_injection_codegen::process_method::process_method;
use margaret_injection_codegen::request_binding_marker::request_binding_marker;
use margaret_item_naming_argument::item_naming_argument::ItemNamingArgument;

use crate::first_tick::FirstTick;
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

fn canonical_attribute_path(
    index: &AttributeIndex,
    item: &IndexedItem,
    written: &Path,
    argument: &'static str,
) -> Result<CanonicalPath, ServiceCodegenError> {
    index.resolve_item_path(item, written).ok_or_else(|| {
        ServiceCodegenError::UnresolvedTickerPath {
            argument,
            path: format_path(written),
            ticker: item.canonical_path().to_string(),
        }
    })
}

fn validate_runner(
    index: &AttributeIndex,
    item: &IndexedItem,
    runner: &IndexedMethod,
    path: &str,
) -> Result<(), ServiceCodegenError> {
    for view in parameters(runner) {
        if let Some(name) = request_binding_marker(view.attributes) {
            return Err(ServiceCodegenError::RunnerRequestBinding {
                path: path.to_string(),
                parameter: view.holder.to_string(),
                marker: name.name().to_string(),
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
            let behavior = behavior
                .as_ref()
                .map(|written| {
                    canonical_attribute_path(
                        index,
                        item,
                        written,
                        ItemNamingArgument::TickBehavior.key(),
                    )
                })
                .transpose()?;
            let interval = canonical_attribute_path(
                index,
                item,
                &interval,
                ItemNamingArgument::TickInterval.key(),
            )?;

            ServiceKind::Ticker {
                behavior,
                first_tick: FirstTick::Immediate,
                interval,
            }
        }
    };

    Ok(ServiceUnit {
        concrete_path: item.canonical_path().clone(),
        field_name: identifier.field().to_string(),
        is_async: runner.signature().asyncness.is_some(),
        kind,
        origin: ServiceUnitOrigin::User,
        runner: runner.identifier().to_string(),
        takes_token,
        type_name: identifier.type_name().to_string(),
    })
}

fn has_conflicting_roles(item: &IndexedItem, role: Role) -> bool {
    let others = match role {
        Role::Service => [
            FrameworkAttribute::ConsoleCommand,
            FrameworkAttribute::ScheduledWithTickTimer,
        ],
        Role::Ticker => [
            FrameworkAttribute::ConsoleCommand,
            FrameworkAttribute::Service,
        ],
    };

    others
        .iter()
        .any(|other| item.has_framework_attribute(*other))
}

fn runner_takes_token(index: &AttributeIndex, item: &IndexedItem, method: &IndexedMethod) -> bool {
    parameters(method)
        .iter()
        .any(|view| is_cancellation_token(index, item, view.declared))
}

pub(crate) fn service_units(
    index: &AttributeIndex,
) -> Result<Vec<ServiceUnit>, ServiceCodegenError> {
    let mut units = Vec::new();

    for matched in index.select_framework_attribute(FrameworkAttribute::Service) {
        units.push(build_unit(&matched, index, Role::Service)?);
    }

    for matched in index.select_framework_attribute(FrameworkAttribute::ScheduledWithTickTimer) {
        units.push(build_unit(&matched, index, Role::Ticker)?);
    }

    Ok(units)
}
