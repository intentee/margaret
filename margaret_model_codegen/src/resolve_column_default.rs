use margaret_model::column_default::ColumnDefault;
use margaret_model::column_type::ColumnType;

pub(crate) fn resolve_column_default(
    column_type: ColumnType,
    primary_key: bool,
    foreign_key: bool,
) -> ColumnDefault {
    if column_type == ColumnType::Uuid && primary_key && !foreign_key {
        ColumnDefault::UuidV7
    } else {
        ColumnDefault::NotSet
    }
}

#[cfg(test)]
mod tests {
    use margaret_model::column_default::ColumnDefault;
    use margaret_model::column_type::ColumnType;

    use super::resolve_column_default;

    #[test]
    fn a_uuid_primary_key_auto_generates() {
        assert_eq!(
            resolve_column_default(ColumnType::Uuid, true, false),
            ColumnDefault::UuidV7
        );
    }

    #[test]
    fn a_uuid_foreign_key_that_is_also_a_primary_key_is_not_generated() {
        assert_eq!(
            resolve_column_default(ColumnType::Uuid, true, true),
            ColumnDefault::NotSet
        );
    }

    #[test]
    fn a_non_primary_key_uuid_is_not_generated() {
        assert_eq!(
            resolve_column_default(ColumnType::Uuid, false, false),
            ColumnDefault::NotSet
        );
    }

    #[test]
    fn a_non_uuid_primary_key_is_not_generated() {
        assert_eq!(
            resolve_column_default(ColumnType::Text, true, false),
            ColumnDefault::NotSet
        );
    }
}
