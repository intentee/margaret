use std::sync::LazyLock;

use margaret_attributes::canonical_path::CanonicalPath;
use margaret_attributes::framework_vocabulary::FrameworkVocabulary;
use margaret_model::column_default::ColumnDefault;
use margaret_model::on_delete::OnDelete;

fn column_default_name(column_default: ColumnDefault) -> &'static str {
    match column_default {
        ColumnDefault::NotSet => "NotSet",
        ColumnDefault::UuidV7 => "UuidV7",
    }
}

fn on_delete_name(on_delete: OnDelete) -> &'static str {
    match on_delete {
        OnDelete::Cascade => "Cascade",
        OnDelete::NoAction => "NoAction",
        OnDelete::Restrict => "Restrict",
        OnDelete::SetNull => "SetNull",
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

pub(crate) static DATE_TIME_CANONICAL_PATH: LazyLock<CanonicalPath> =
    LazyLock::new(|| CanonicalPath::new(vec!["chrono".to_string(), "DateTime".to_string()]));

pub(crate) static DECIMAL_CANONICAL_PATH: LazyLock<CanonicalPath> =
    LazyLock::new(|| CanonicalPath::new(vec!["rust_decimal".to_string(), "Decimal".to_string()]));

pub static JSON_CANONICAL_PATH: LazyLock<CanonicalPath> = LazyLock::new(|| {
    CanonicalPath::new(
        ["margaret", "framework", "active_record", "json", "Json"]
            .into_iter()
            .map(ToString::to_string)
            .collect(),
    )
});

pub static KEY_CANONICAL_PATH: LazyLock<CanonicalPath> = LazyLock::new(|| {
    CanonicalPath::new(
        ["margaret", "framework", "active_record", "key", "Key"]
            .into_iter()
            .map(ToString::to_string)
            .collect(),
    )
});

pub const ON_DELETE_ACTIONS: FrameworkVocabulary<OnDelete> = FrameworkVocabulary {
    enum_path: &["margaret", "framework", "model", "on_delete", "OnDelete"],
    name: on_delete_name,
    variants: &[OnDelete::Cascade, OnDelete::Restrict, OnDelete::SetNull],
};

pub(crate) static SECRET_TEXT_CANONICAL_PATH: LazyLock<CanonicalPath> = LazyLock::new(|| {
    CanonicalPath::new(
        [
            "margaret",
            "framework",
            "active_record",
            "secret_text",
            "SecretText",
        ]
        .into_iter()
        .map(ToString::to_string)
        .collect(),
    )
});

pub(crate) static UTC_CANONICAL_PATH: LazyLock<CanonicalPath> =
    LazyLock::new(|| CanonicalPath::new(vec!["chrono".to_string(), "Utc".to_string()]));

pub(crate) static UUID_CANONICAL_PATH: LazyLock<CanonicalPath> =
    LazyLock::new(|| CanonicalPath::new(vec!["uuid".to_string(), "Uuid".to_string()]));
