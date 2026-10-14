use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use margaret_container_tests::generate_container_modules::generate_container_modules;

#[test]
fn separates_the_arguments_of_a_builder_from_a_command_named_after_them() {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/arguments_named_like_a_command");
    let modules = generate_container_modules("crate", &directory)
        .expect("the crate with commands named after arguments renders");
    let files: BTreeSet<PathBuf> = modules
        .iter()
        .map(|module| {
            module
                .path()
                .directory_in(Path::new(""))
                .join(module.path().file_name())
        })
        .collect();

    assert_eq!(files.len(), modules.len());
}
