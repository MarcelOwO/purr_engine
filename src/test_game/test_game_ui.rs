use purrengine::engine_core::module::Module;
use purrengine::engine_scene::scene::Scene;

pub(crate) struct TestGameUI {}

impl Default for TestGameUI {
    fn default() -> Self {
        Self{}
    }
}

impl Module for TestGameUI {

    fn get_scene(&mut self) -> Scene {
        todo!()
    }
}
