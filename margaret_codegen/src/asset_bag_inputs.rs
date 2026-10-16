use std::path::Path;

#[derive(Debug)]
pub enum AssetBagInputs {
    Metafile,
    MetafileAndAssetsDirectory,
    Unread,
}

impl AssetBagInputs {
    pub(crate) fn read_paths<'paths>(
        &self,
        metafile: &'paths Path,
        assets_directory: &'paths Path,
    ) -> Vec<&'paths Path> {
        match self {
            Self::Metafile => vec![metafile],
            Self::MetafileAndAssetsDirectory => vec![metafile, assets_directory],
            Self::Unread => Vec::new(),
        }
    }
}
