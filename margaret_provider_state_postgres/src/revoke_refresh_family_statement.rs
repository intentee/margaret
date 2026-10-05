use crate::refresh_families_table::REFRESH_FAMILIES_TABLE;

pub(crate) fn revoke_refresh_family_statement() -> String {
    let families = REFRESH_FAMILIES_TABLE;

    format!(r#"DELETE FROM "{families}" WHERE "id" = $1"#)
}
