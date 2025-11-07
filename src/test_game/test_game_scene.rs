use purrengine::engine_core::module::Module;
use purrengine::engine_entities::actors::Actor;
use purrengine::engine_scene::scene::Scene;

pub struct TestGameScene {
    actors: Vec<Actor>,
}
impl Default for TestGameScene {
    fn default() -> Self {
        todo!()
    }
}

impl Module for TestGameScene {
    fn get_scene(&mut self) -> Scene {
        todo!()
    }
}
