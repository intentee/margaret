use std::collections::BTreeSet;

pub struct ServedAssets {
    pub assets_directory_name: String,
    pub served_tails: BTreeSet<String>,
}
