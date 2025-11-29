use crate::{engine_core::identifier::uuid, engine_entities::actors::Actor};
use std::collections::{HashMap, HashSet};

pub(crate) struct Scene {
    actors: HashSet<u64>,
}

impl Scene {
    pub(crate) fn new() -> Self {
        Self {
            actors: HashSet::new(),
        }
    }

    pub(crate) fn add_actor(&mut self, id: u64) {
        self.actors.insert(id);
    }

    pub(crate) fn delete_actor(&mut self, id: &u64) {
        self.actors.remove(id);
    }

    pub(crate) fn mut_actors(&mut self, mut f: impl FnMut(u64)) {
        for id in self.actors.iter() {
            f(*id);
        }
    }
}
