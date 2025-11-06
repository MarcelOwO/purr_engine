use crate::engine_entities::actors::Actor;

pub trait Scene {

    fn get_actors(&mut self) -> &mut Vec<Actor>;
    fn add_actor(&mut self, actor: Actor);
}