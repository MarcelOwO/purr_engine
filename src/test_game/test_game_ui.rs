use purrengine::engine_core::module::Module;

pub(crate) struct TestGameUI {}

impl Default for TestGameUI {
    fn default() -> Self {
        Self {}
    }
}

impl Module for TestGameUI {
    fn setup(&mut self) {}
    fn get_scene(&mut self) {}
}
