use purrengine::engine_core::module::Module;

pub struct TestGameScene {}
impl Default for TestGameScene {
    fn default() -> Self {
        Self {}
    }
}

impl Module for TestGameScene {
    fn setup(&mut self) {}
    fn get_scene(&mut self) {}
}
