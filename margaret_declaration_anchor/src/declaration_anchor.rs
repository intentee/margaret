use margaret_attributes::attribute_index::AttributeIndex;
use margaret_attributes::framework_attribute::FrameworkAttribute;
use margaret_attributes::matched_attribute::MatchedAttribute;

use crate::declaration_anchor_error::DeclarationAnchorError;
use crate::declaration_attributes::DECLARATION_ATTRIBUTES;
use crate::declared_anchor::DeclaredAnchor;

/// # Errors
///
/// Returns `DeclarationAnchorError` when the declaration is not anchored by a plain struct that
/// carries no other declaration.
pub fn declaration_anchor<'index>(
    index: &'index AttributeIndex,
    matched: &MatchedAttribute<'index>,
    attribute: FrameworkAttribute,
) -> Result<DeclaredAnchor<'index>, DeclarationAnchorError> {
    let item = matched.item();
    let path = item.canonical_path().to_string();
    let Some(identifier) = index.struct_identifier(item.canonical_path()) else {
        return Err(DeclarationAnchorError::NotAStruct {
            attribute: attribute.name(),
            path,
        });
    };

    if item.has_framework_attribute(FrameworkAttribute::Singleton) {
        return Err(DeclarationAnchorError::DeclaredAsSingleton {
            attribute: attribute.name(),
            path,
        });
    }

    let declarations = item
        .attributes()
        .iter()
        .filter(|indexed| {
            indexed
                .framework_attribute()
                .is_some_and(|framework| DECLARATION_ATTRIBUTES.contains(&framework))
        })
        .count();

    if declarations > 1 {
        return Err(DeclarationAnchorError::SharedAnchor { declarations, path });
    }

    Ok(DeclaredAnchor { identifier, item })
}

#[cfg(test)]
mod tests {
    use margaret_attributes::framework_attribute::FrameworkAttribute;
    use margaret_attributes_tests::indexed_source::IndexedSource;

    use super::declaration_anchor;
    use crate::declaration_anchor_error::DeclarationAnchorError;

    fn anchored(source: &str) -> Result<String, DeclarationAnchorError> {
        let indexed = IndexedSource::new(source);
        let matched = indexed
            .index
            .select_framework_attribute(FrameworkAttribute::IssuesTokens)
            .next()
            .expect("the declaration is indexed");

        declaration_anchor(&indexed.index, &matched, FrameworkAttribute::IssuesTokens)
            .map(|anchor| format!("{} {}", anchor.item.identifier(), anchor.identifier.field()))
    }

    #[test]
    fn accepts_a_plain_struct() {
        assert_eq!(
            anchored("#[issues_tokens]\npub struct Issuer;\n").expect("the struct anchors"),
            "Issuer issuer"
        );
    }

    #[test]
    fn rejects_a_declaration_on_an_enum() {
        assert!(matches!(
            anchored("#[issues_tokens]\npub enum Issuer { Only }\n"),
            Err(DeclarationAnchorError::NotAStruct { attribute: "issues_tokens", path })
                if path == "crate::Issuer"
        ));
    }

    #[test]
    fn rejects_a_declaration_on_a_singleton() {
        assert!(matches!(
            anchored("#[singleton]\n#[issues_tokens]\npub struct Issuer;\n"),
            Err(DeclarationAnchorError::DeclaredAsSingleton {
                attribute: "issues_tokens",
                ..
            })
        ));
    }

    #[test]
    fn rejects_a_struct_anchoring_two_declarations() {
        assert!(matches!(
            anchored("#[issues_tokens]\n#[verifies_tokens_from_issuer(auth)]\npub struct Issuer;\n"),
            Err(DeclarationAnchorError::SharedAnchor { declarations, path })
                if declarations == 2 && path == "crate::Issuer"
        ));
    }
}
