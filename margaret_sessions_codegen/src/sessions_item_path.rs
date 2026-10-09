use margaret_attributes::canonical_path::CanonicalPath;
use margaret_umbrella_path::umbrella_item_path::umbrella_item_path;

use crate::sessions_item::SessionsItem;
use crate::sessions_module_name::SESSIONS_MODULE_NAME;

#[must_use]
pub fn sessions_item_path(item: SessionsItem) -> CanonicalPath {
    umbrella_item_path(&[SESSIONS_MODULE_NAME, item.type_name()])
}

#[cfg(test)]
mod tests {
    use super::sessions_item_path;
    use crate::sessions_item::SessionsItem;

    #[test]
    fn places_each_sessions_item_in_the_sessions_module() {
        assert_eq!(
            [
                SessionsItem::ConsumedSessions,
                SessionsItem::IssuedSessions,
                SessionsItem::SessionRefreshEndpoint,
                SessionsItem::SessionSignOutEndpoint,
            ]
            .map(|item| sessions_item_path(item).to_string()),
            [
                "crate::margaret::sessions::ConsumedSessions",
                "crate::margaret::sessions::IssuedSessions",
                "crate::margaret::sessions::SessionRefreshEndpoint",
                "crate::margaret::sessions::SessionSignOutEndpoint",
            ]
        );
    }
}
