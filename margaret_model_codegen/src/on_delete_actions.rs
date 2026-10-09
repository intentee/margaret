use margaret_attributes::framework_vocabulary::FrameworkVocabulary;
use margaret_model::on_delete::OnDelete;

fn on_delete_name(on_delete: OnDelete) -> &'static str {
    match on_delete {
        OnDelete::Cascade => "Cascade",
        OnDelete::NoAction => "NoAction",
        OnDelete::Restrict => "Restrict",
        OnDelete::SetDefault => "SetDefault",
        OnDelete::SetNull => "SetNull",
    }
}

pub const ON_DELETE_ACTIONS: FrameworkVocabulary<OnDelete> = FrameworkVocabulary {
    enum_path: &["margaret", "framework", "model", "on_delete", "OnDelete"],
    name: on_delete_name,
    variants: &[
        OnDelete::Cascade,
        OnDelete::Restrict,
        OnDelete::SetDefault,
        OnDelete::SetNull,
    ],
};
