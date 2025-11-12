use crate::engine_entities::actors::Actor;
use std::collections::HashMap;

pub struct Scene {
    actors: HashMap<u64, Actor>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            actors: HashMap::new(),
        }
    }
    pub(crate) fn mutate(&mut self, mut f: impl FnMut(&mut Actor)) {
        for actor in self.actors.values_mut() {
            f(actor);
        }
    }

    pub fn add_actor(&mut self, id: u64, actor: Actor) {
        self.actors.insert(id, actor);
    }

    pub fn get_actor(&self, id: u64) -> &Actor {
        self.actors.get(&id).unwrap()
    }
}
