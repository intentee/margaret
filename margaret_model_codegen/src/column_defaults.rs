use margaret_attributes::framework_vocabulary::FrameworkVocabulary;
use margaret_model::column_default::ColumnDefault;

fn column_default_name(column_default: ColumnDefault) -> &'static str {
    match column_default {
        ColumnDefault::NotSet => "NotSet",
        ColumnDefault::UuidV7 => "UuidV7",
    }
}

pub const COLUMN_DEFAULTS: FrameworkVocabulary<ColumnDefault> = FrameworkVocabulary {
    enum_path: &[
        "margaret",
        "framework",
        "model",
        "column_default",
        "ColumnDefault",
    ],
    name: column_default_name,
    variants: &[ColumnDefault::UuidV7],
};
