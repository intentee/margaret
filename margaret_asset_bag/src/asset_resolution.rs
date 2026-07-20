#[derive(Clone, Copy)]
pub struct AssetResolution<TBundle, TImage, TFile> {
    pub(crate) bundle: TBundle,
    pub(crate) image: TImage,
    pub(crate) file: TFile,
}

impl<TBundle, TImage, TFile> AssetResolution<TBundle, TImage, TFile> {
    #[must_use]
    pub const fn new(bundle: TBundle, image: TImage, file: TFile) -> Self {
        Self {
            bundle,
            image,
            file,
        }
    }
}
