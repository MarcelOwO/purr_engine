use crate::engine_app::app::App;
use crate::engine_assets::asset::Asset;
use crate::engine_assets::asset_manager::AssetManager;
use crate::engine_core::settings::Settings;
use crate::engine_scene::scene::Scene;

//scuffed builder pattern for app
pub struct Builder {
    app: App,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            app: App::new(Settings::default()),
        }
    }

    pub fn configure(&mut self, mut f: impl FnMut(&mut Settings)) -> &mut Self {
        let mut settings = Settings::default();
        f(&mut settings);
        self.app.update_setting(settings);
        self
    }
    pub fn add_assets(&mut self, mut f: impl FnMut(&mut AssetManager)) -> &mut Self {
        self.app.mut_asset_manager(|asset_manager| f(asset_manager));
        self
    }

    pub fn add_scene(&mut self, mut f: impl FnMut(&mut App, u64)) -> &mut Self {
        let scene_id = self.app.create_scene();
        f(&mut self.app, scene_id);
        self
    }

    //start and run the app
    pub fn run(&mut self) {
        self.app.run();
    }
}
