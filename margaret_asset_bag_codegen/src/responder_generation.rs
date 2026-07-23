use std::collections::BTreeSet;

pub enum ResponderGeneration {
    Emit {
        assets_directory_name: String,
        embed_relative: String,
        served_tails: BTreeSet<String>,
    },
    Skip,
}
