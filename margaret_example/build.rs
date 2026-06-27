use std::fs;
use std::path::Path;

use margaret_attributes::crate_root::CrateRoot;

fn main() {
    let manifest_directory = env!("CARGO_MANIFEST_DIR");
    let host_source = Path::new(manifest_directory).join("src");
    let generated_directory = host_source.join("margaret");
    let plugin_source = Path::new(manifest_directory).join("../margaret_example_plugin/src");

    let crates = [
        CrateRoot::new("crate", host_source),
        CrateRoot::new("margaret_example_plugin", plugin_source),
    ];

    fs::create_dir_all(&generated_directory).expect("the generated directory is created");

    let umbrella = generated_directory.join("mod.rs");

    if !umbrella.exists() {
        fs::write(&umbrella, "").expect("the umbrella stub is written");
    }

    margaret_codegen::build::build(&crates)
        .expect("the example is generated")
        .write_to(&generated_directory);

    for crate_root in &crates {
        println!(
            "cargo:rerun-if-changed={}",
            crate_root.source_directory.display()
        );
    }
}
