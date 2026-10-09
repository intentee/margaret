use std::collections::HashMap;

use margaret_database::executor::Executor;
use margaret_sql::select_limit::SelectLimit;

use crate::active_record_error::ActiveRecordError;
use crate::children::Children;
use crate::completeness::Completeness;
use crate::lateral_children::LateralChildren;
use crate::loadable::Loadable;
use crate::loaded_rows::LoadedRows;
use crate::many_relation::ManyRelation;
use crate::one_relation::OneRelation;

pub struct ChildGroups<Child> {
    groups: HashMap<i64, Vec<Child>>,
    limit: usize,
}

impl<Child: Loadable> ChildGroups<Child> {
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the children cannot be read.
    pub async fn many<Executing: Executor>(
        rows: &LoadedRows<'_>,
        key_offset: usize,
        ManyRelation { key, limit, order }: &ManyRelation,
        executor: &Executing,
    ) -> Result<Self, ActiveRecordError> {
        LateralChildren {
            key: *key,
            limit: SelectLimit::Rows(*limit as u64 + 1),
            order,
        }
        .grouped(rows, key_offset, executor)
        .await
        .map(|groups| Self {
            groups,
            limit: *limit,
        })
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the child cannot be read.
    pub async fn one<Executing: Executor>(
        rows: &LoadedRows<'_>,
        key_offset: usize,
        OneRelation { key }: &OneRelation,
        executor: &Executing,
    ) -> Result<Self, ActiveRecordError> {
        LateralChildren {
            key: *key,
            limit: SelectLimit::Unlimited,
            order: &[],
        }
        .grouped(rows, key_offset, executor)
        .await
        .map(|groups| Self { groups, limit: 1 })
    }

    pub fn child(&mut self, parent: i64) -> Option<Child> {
        self.groups
            .remove(&parent)
            .and_then(|children| children.into_iter().next())
    }

    pub fn children(&mut self, parent: i64) -> Children<Child> {
        let mut records = self.groups.remove(&parent).unwrap_or_default();

        if records.len() > self.limit {
            records.truncate(self.limit);

            Children {
                completeness: Completeness::Truncated,
                records,
            }
        } else {
            Children {
                completeness: Completeness::Complete,
                records,
            }
        }
    }
}
