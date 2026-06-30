use margaret_attributes::attribute_index::AttributeIndex;

use crate::append_synthetic_providers::append_synthetic_providers;
use crate::build_plan::build_plan;
use crate::container_error::ContainerError;
use crate::generated_source::GeneratedSource;
use crate::render::render;
use crate::synthetic_provider::SyntheticProvider;
use crate::topological_order::topological_order;

pub fn render_container(
    index: &AttributeIndex,
    synthetic: &[SyntheticProvider],
) -> Result<GeneratedSource, ContainerError> {
    let mut plan = build_plan(index, synthetic)?;

    append_synthetic_providers(&mut plan, synthetic)?;
    topological_order(&plan.providers, &plan.collections)?;

    Ok(GeneratedSource::new(render(&plan)))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use margaret_attributes::attribute_index::AttributeIndex;
    use margaret_attributes::attribute_index_builder::AttributeIndexBuilder;
    use margaret_attributes::canonical_path::CanonicalPath;
    use margaret_attributes::crate_root::CrateRoot;

    use crate::container_error::ContainerError;
    use crate::synthetic_provider::SyntheticProvider;

    use super::render_container;

    const STORE: &str = "#[singleton]\nstruct Store;\n\nimpl Store {\n    #[constructor]\n    fn create() -> Self {}\n}\n";

    fn index_for(source: &str) -> AttributeIndex {
        let directory = tempdir().expect("a temporary crate directory");
        let source_directory = directory.path().join("src");

        fs::create_dir(&source_directory).expect("the src directory is created");
        fs::write(source_directory.join("lib.rs"), source).expect("lib.rs is written");

        AttributeIndexBuilder::new()
            .index_crate(&CrateRoot::new("crate", &source_directory))
            .expect("the crate is indexed")
            .build()
    }

    fn path(segments: &[&str]) -> CanonicalPath {
        CanonicalPath::new(segments.iter().map(|segment| segment.to_string()).collect())
    }

    fn gatekeeper(
        concrete: &[&str],
        field_name: &str,
        dependencies: Vec<CanonicalPath>,
    ) -> SyntheticProvider {
        SyntheticProvider {
            concrete_path: path(concrete),
            constructor: "new".to_string(),
            dependencies,
            field_name: field_name.to_string(),
        }
    }

    fn error_for(index: &AttributeIndex, synthetic: SyntheticProvider) -> ContainerError {
        render_container(index, &[synthetic])
            .err()
            .expect("the synthetic provider must be rejected")
    }

    #[test]
    fn wires_a_synthetic_provider_from_a_singleton() {
        let index = index_for(STORE);
        let synthetic = gatekeeper(
            &["crate", "margaret", "security", "Gatekeeper"],
            "gatekeeper",
            vec![path(&["crate", "Store"])],
        );

        let source = render_container(&index, &[synthetic])
            .expect("the container renders")
            .source()
            .to_string();

        assert!(source.contains("pub async fn gatekeeper"));
        assert!(source.contains("crate::margaret::security::Gatekeeper::new(self.store().await)"));
    }

    #[test]
    fn lets_a_singleton_depend_on_a_synthetic_provider() {
        let index = index_for(
            "#[singleton]\nstruct Consumer;\n\nimpl Consumer {\n    #[constructor]\n    fn create(gatekeeper: std::sync::Arc<crate::margaret::security::Gatekeeper>) -> Self {}\n}\n",
        );
        let synthetic = gatekeeper(
            &["crate", "margaret", "security", "Gatekeeper"],
            "gatekeeper",
            Vec::new(),
        );

        let source = render_container(&index, &[synthetic])
            .expect("the container renders")
            .source()
            .to_string();

        assert!(source.contains("crate::Consumer::create(self.gatekeeper().await)"));
    }

    #[test]
    fn rejects_a_synthetic_provider_that_conflicts_with_a_singleton() {
        let index = index_for(STORE);
        let synthetic = gatekeeper(&["crate", "Store"], "gatekeeper", Vec::new());

        assert!(
            error_for(&index, synthetic)
                .to_string()
                .contains("collides with singleton")
        );
    }

    #[test]
    fn rejects_a_synthetic_provider_with_a_field_collision() {
        let index = index_for(STORE);
        let synthetic = gatekeeper(
            &["crate", "margaret", "security", "Gatekeeper"],
            "store",
            Vec::new(),
        );

        assert!(
            error_for(&index, synthetic)
                .to_string()
                .contains("maps to container field")
        );
    }

    #[test]
    fn rejects_a_synthetic_provider_with_a_missing_dependency() {
        let index = index_for(STORE);
        let synthetic = gatekeeper(
            &["crate", "margaret", "security", "Gatekeeper"],
            "gatekeeper",
            vec![path(&["crate", "Missing"])],
        );

        assert!(
            error_for(&index, synthetic)
                .to_string()
                .contains("no singleton provides")
        );
    }
}
