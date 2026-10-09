use crate::index_redundancy::IndexRedundancy;
use crate::resolved_index::ResolvedIndex;
use crate::resolved_unique_constraint::ResolvedUniqueConstraint;

pub(crate) fn redundant_index(
    index: &ResolvedIndex,
    primary_key: &[String],
    unique_constraints: &[ResolvedUniqueConstraint],
) -> Option<IndexRedundancy> {
    if primary_key.starts_with(&index.columns) {
        return Some(IndexRedundancy::PrimaryKeyLeadingColumns);
    }

    if unique_constraints
        .iter()
        .any(|unique_constraint| unique_constraint.columns.starts_with(&index.columns))
    {
        return Some(IndexRedundancy::UniqueConstraint);
    }

    None
}
