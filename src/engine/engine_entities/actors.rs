use std::collections::HashSet;

use crate::engine_entities::transform::Transform;
#[derive(Clone, Default)]
pub struct Actor {
    name: String,
    transform: Transform,
    entities: HashSet<u64>,
}

impl Actor {
    pub fn new() -> Actor {
        Actor {
            name: String::new(),
            entities: HashSet::new(),
            transform: Transform::default(),
        }
    }

    pub(crate) fn mut_components(&mut self, mut f: impl FnMut(u64)) {
        for actor in &mut self.entities.iter() {
            f(*actor);
        }
    }

    pub(crate) fn add_component(&mut self, component_id: u64) {
        self.entities.insert(component_id);
    }
    pub(crate) fn remove_component(&mut self, component_id: u64) {
        self.entities.remove(&component_id);
    }
}
