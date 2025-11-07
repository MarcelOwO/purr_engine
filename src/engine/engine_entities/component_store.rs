use crate::engine_entities::component::Component;
use std::collections::HashMap;

pub(crate) struct ComponentStore {
    components: HashMap<u64, Box<dyn Component>>,
}

impl ComponentStore {
    pub(crate) fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub(crate) fn add_component(&mut self, id: u64, component: Box<dyn Component>) {
        self.components.insert(id, component);
    }
    pub(crate) fn get_component(&mut self, id: u64) -> Option<&mut Box<dyn Component>> {
        self.components.get_mut(&id)
    }
    fn remove_component(&mut self, id: u64) {
        self.components.remove(&id);
    }
}
