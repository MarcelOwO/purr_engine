use purrengine::engine_entities::actors::Actor;
use purrengine::engine_scene::scene::Scene;

pub struct TestScene {
    actors: Vec<Actor>,
}

impl TestScene {
    pub(crate) fn new() -> Self {
        TestScene{ actors: Vec::new()}
    }
}


impl Scene for TestScene{
    fn get_actors(&mut self) -> &mut Vec<Actor> {
        &mut self.actors
    }

    fn add_actor(&mut self, actor: Actor) {
        self.actors.push(actor);
    }
}

