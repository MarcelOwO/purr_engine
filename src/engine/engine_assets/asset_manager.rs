use std::collections::HashMap;
use crate::engine_assets::asset::Asset;
pub(crate) struct AssetManager {
    assets: HashMap<String, Asset>,
}

impl AssetManager {
    pub(crate) fn new() -> Self {
        Self{assets:HashMap::new()}
    }
    fn add(&mut self,key: String, asset: Asset)   {
        self.assets.insert(key, asset);
    }
    fn load_assets(&self) {}

    fn get_asset(&mut self,key: &str) -> Option<&Asset> {
        self.assets.get(key)
    }
    fn remove_asset(&mut self,key: &str) {
        self.assets.remove(key);
    }
}
