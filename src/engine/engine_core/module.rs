use crate::engine_scene::scene::Scene;

pub trait Module {
    fn setup(&mut self) {}

    fn get_scene(&mut self) -> Scene;
}
