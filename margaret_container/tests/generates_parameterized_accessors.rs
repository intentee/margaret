use std::path::Path;

use margaret_attributes::attribute_selector::AttributeSelector;
use margaret_container::generate_container_source;

#[test]
fn generates_parameterized_accessors() {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/parameterized");
    let input_selectors = vec![AttributeSelector::parse("input").expect("a valid selector")];
    let generated = generate_container_source("parameterized", &directory, &input_selectors)
        .expect("the parameterized fixture generates a container");
    let source: String = generated.source().split_whitespace().collect();

    assert!(source.contains("pubfnconcrete(&self,value:String"));
    assert!(source.contains("crate::Concrete::new(self.plain(),value)"));
    assert!(source.contains("pubfnhandler(&self,label:String"));
    assert!(source.contains("->std::sync::Arc<dyncrate::Handler+Send+Sync>"));
    assert!(source.contains("crate::Interfaced::new(label)"));
}
