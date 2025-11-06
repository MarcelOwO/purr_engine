use crate::engine_entities::entitie::Entity;
use crate::engine_entities::transform::Transform;

pub struct Actor {
    transform: Transform,
    pub(crate) entities: Vec<Box<dyn Entity>>,
}

impl Actor {
    fn new() -> Actor {
        Actor {
            transform: Transform::new(),
            entities: Vec::new(),
        }
    }
}
