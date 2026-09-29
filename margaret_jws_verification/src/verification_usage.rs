use std::ops::ControlFlow;

use margaret_jose_parameters::key_operation::KeyOperation;
use margaret_jose_parameters::key_use::KeyUse;

use crate::key_set_composition::KeySetComposition;
use crate::key_usage_rejection::KeyUsageRejection;
use crate::parameter_value::ParameterValue;

fn declared_use(
    key_use: Option<ParameterValue<KeyUse>>,
) -> ControlFlow<KeyUsageRejection, Option<KeyUse>> {
    match key_use {
        Some(ParameterValue::Supported(key_use)) => ControlFlow::Continue(Some(key_use)),
        Some(ParameterValue::Unsupported(key_use)) => {
            ControlFlow::Break(KeyUsageRejection::UnrecognizedUse { key_use })
        }
        None => ControlFlow::Continue(None),
    }
}

fn declared_operations(
    key_ops: Vec<ParameterValue<KeyOperation>>,
    declared_use: Option<KeyUse>,
) -> ControlFlow<KeyUsageRejection, Vec<KeyOperation>> {
    let mut operations = Vec::with_capacity(key_ops.len());

    for key_op in key_ops {
        let key_op =
            key_op.supported(|key_op| KeyUsageRejection::UnrecognizedOperation { key_op })?;

        if operations.contains(&key_op) {
            return ControlFlow::Break(KeyUsageRejection::DuplicateOperation { key_op });
        }

        if let Some(key_use) = declared_use
            && key_op.key_use() != key_use
        {
            return ControlFlow::Break(KeyUsageRejection::OperationContradictsUse {
                key_op,
                key_use,
            });
        }

        operations.push(key_op);
    }

    ControlFlow::Continue(operations)
}

fn verifying_operations(operations: Vec<KeyOperation>) -> ControlFlow<KeyUsageRejection> {
    if !operations.contains(&KeyOperation::Verify) {
        return ControlFlow::Break(KeyUsageRejection::VerificationNotPermitted {
            key_ops: operations,
        });
    }

    match operations
        .into_iter()
        .find(|key_op| key_op.key_use() != KeyUse::Signature)
    {
        Some(key_op) => ControlFlow::Break(KeyUsageRejection::UnrelatedOperation { key_op }),
        None => ControlFlow::Continue(()),
    }
}

pub(crate) fn verification_usage(
    key_use: Option<ParameterValue<KeyUse>>,
    key_ops: Option<Vec<ParameterValue<KeyOperation>>>,
    composition: KeySetComposition,
) -> ControlFlow<KeyUsageRejection> {
    let declared_use = declared_use(key_use)?;
    let operations = match key_ops {
        Some(key_ops) => Some(declared_operations(key_ops, declared_use)?),
        None => None,
    };

    if declared_use == Some(KeyUse::Encryption) {
        return ControlFlow::Break(KeyUsageRejection::EncryptionUse);
    }

    if let Some(operations) = operations {
        verifying_operations(operations)?;
    }

    if declared_use.is_none() && composition == KeySetComposition::IncludesEncryptionKeys {
        return ControlFlow::Break(KeyUsageRejection::MissingUseAmongEncryptionKeys);
    }

    ControlFlow::Continue(())
}
