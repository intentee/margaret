use serde::Serialize;
use serde_json::Map;
use serde_json::Value;
use serde_json::map::Entry;

use crate::claims_merge_error::ClaimsMergeError;

fn merged(
    mut members: Map<String, Value>,
    application_claims: Map<String, Value>,
) -> Result<Map<String, Value>, ClaimsMergeError> {
    for (member, value) in application_claims {
        match members.entry(member) {
            Entry::Occupied(occupied) => {
                return Err(ClaimsMergeError::Colliding {
                    member: occupied.key().clone(),
                });
            }
            Entry::Vacant(vacant) => {
                vacant.insert(value);
            }
        }
    }

    Ok(members)
}

/// # Errors
///
/// Returns `ClaimsMergeError` when the application claims cannot be serialized to json, are not
/// a json object, or name a member that is already present.
pub fn merge_claims<TClaims: Serialize>(
    members: Map<String, Value>,
    application_claims: &TClaims,
) -> Result<Map<String, Value>, ClaimsMergeError> {
    serde_json::to_value(application_claims)
        .map_err(ClaimsMergeError::Serialization)
        .and_then(|serialized| match serialized {
            Value::Object(application_claims) => merged(members, application_claims),
            Value::Array(_)
            | Value::Bool(_)
            | Value::Null
            | Value::Number(_)
            | Value::String(_) => Err(ClaimsMergeError::NotAnObject),
        })
}
