use crate::engine_entities::transform::Transform;

pub struct Actor {
    transform: Transform,
    name: String,
    is_static: bool,
    is_active: bool,
    pub(crate) entities: Vec<u64>,
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

    pub fn add_component(&mut self, component: u64) {
        self.entities.push(component);
    }
}
