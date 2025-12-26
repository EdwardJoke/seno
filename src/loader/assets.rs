use super::Loader;

impl Loader {
    pub fn assets<const N: usize>(mut self, assets: [&'static str; N]) -> Self {
        self.assets = assets.to_vec();
        self
    }
}