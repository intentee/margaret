use std::path::Path;

use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::crate_root::CrateRoot;
use margaret_console_argument_codegen::scan::scan;
use margaret_container::plan_container::plan_container;
use margaret_container::planned_container::PlannedContainer;

fn planned_container() -> PlannedContainer {
    let directory =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/console_argument_propagation");
    let index = AttributeIndexBuilder::new()
        .index_crate(&CrateRoot::new("crate", directory))
        .expect("the fixture crate is indexed")
        .build();
    let registry = scan(&index).expect("the console arguments are scanned");
    plan_container(&index, &registry, &[]).expect("the container is planned")
}

fn missing_path() -> CanonicalPath {
    CanonicalPath::new(vec!["crate".to_string(), "Missing".to_string()])
}

#[test]
fn reports_an_unplanned_construction_root() {
    let planned = planned_container();
    let error = match planned.render(&[missing_path()], &[], &[]) {
        Ok(_) => panic!("an unplanned construction root must not be rendered"),
        Err(error) => error,
    };

    assert!(error.to_string().contains("crate::Missing"));
}

#[test]
fn reports_an_unplanned_retained_root() {
    let planned = planned_container();
    let roots = planned.roots();
    let error = match planned.render(&roots, &[missing_path()], &[]) {
        Ok(_) => panic!("an unplanned retained root must not be rendered"),
        Err(error) => error,
    };

    assert!(error.to_string().contains("crate::Missing"));
}

#[test]
fn reports_an_unplanned_builder_root() {
    let planned = planned_container();
    let roots = planned.roots();
    let error = match planned.render(&roots, &roots, &[missing_path()]) {
        Ok(_) => panic!("an unplanned builder root must not be rendered"),
        Err(error) => error,
    };

    assert!(error.to_string().contains("crate::Missing"));
}
