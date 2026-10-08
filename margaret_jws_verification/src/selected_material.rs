use crate::key_selection::KeySelection;
use crate::verification_material::VerificationMaterial;

pub(crate) struct SelectedMaterial<'set> {
    pub(crate) material: &'set VerificationMaterial,
    pub(crate) selection: KeySelection,
}
