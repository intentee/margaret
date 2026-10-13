use margaret_attributes::indexed_item::IndexedItem;
use margaret_oauth_vocabulary::scope::Scope;

pub struct DeclaredScopeItem<'index> {
    pub anchor: &'index IndexedItem,
    pub scope: Scope,
}
