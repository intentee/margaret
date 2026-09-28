use std::collections::BTreeSet;

pub struct ServedAssets {
    pub assets_directory_name: String,
    pub embed_relative: String,
    pub served_tails: BTreeSet<String>,
}
