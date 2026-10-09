use margaret_attributes::framework_vocabulary::FrameworkVocabulary;

use crate::declared_consent::DeclaredConsent;

fn consent_policy_name(consent: DeclaredConsent) -> &'static str {
    match consent {
        DeclaredConsent::Implicit => "Implicit",
        DeclaredConsent::Prompted => "Prompted",
    }
}

pub(crate) const CONSENT_POLICIES: FrameworkVocabulary<DeclaredConsent> = FrameworkVocabulary {
    enum_path: &[
        "margaret",
        "framework",
        "accepted_clients",
        "consent_policy",
        "ConsentPolicy",
    ],
    name: consent_policy_name,
    variants: &[DeclaredConsent::Implicit, DeclaredConsent::Prompted],
};
