use crate::engine_entities::transform::Transform;

pub struct Actor {
    name: String,
    transform: Transform,
    is_static: bool,
    is_active: bool,
    entities: Vec<u64>,
}

impl Actor {
    pub fn new() -> Actor {
        Actor {
            transform: Transform::new(),
            entities: Vec::new(),
            name: String::from("Unnamed Actor"),
            is_static: false,
            is_active: true,
        }
    }
    pub(crate) fn mutate(&mut self, mut f: impl FnMut(u64)) {
        for actor in self.entities.clone() {
            f(actor);
        }
    }

    pub fn add_component(&mut self, component: u64) {
        self.entities.push(component);
    }

    pub fn get_components(&mut self) -> &mut Vec<u64> {
        &mut self.entities
    }
}
