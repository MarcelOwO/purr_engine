use crate::{engine_core::identifier::uuid, engine_entities::actors::Actor};
use std::collections::HashMap;

pub struct Scene {
    actors: HashMap<u64, Actor>,
}

impl Clone for Scene {
    fn clone(&self) -> Self {
        Self {
            actors: self.actors.clone(),
        }
    }
}

impl Scene {
    pub fn new() -> Self {
        Self {
            actors: HashMap::new(),
        }
    }
    pub(crate) fn mut_actors(&mut self, mut f: impl FnMut(&mut Actor)) {
        for actor in self.actors.values_mut() {
            f(actor);
        }
    }

    pub fn add_actor(&mut self, mut f: impl FnMut(&mut Actor)) -> &mut Self {
        let mut actor = Actor::new();
        f(&mut actor);
        let id = uuid::get_uuid();
        self.actors.insert(id, actor);
        self
    }

    pub fn get_actor(&self, id: u64) -> &Actor {
        self.actors.get(&id).unwrap()
    }
}
