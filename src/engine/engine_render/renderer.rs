use crate::engine_core::frame_data::FrameData;
use crate::engine_entities::component::Component;
use crate::engine_entities::component_store::ComponentStore;
use crate::engine_entities::entity_type::ComponentType;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct Renderer {
    components: Vec<u64>,
}

impl Renderer {
    pub(crate) fn register_components(&mut self, components: Vec<u64>) {
        self.components = components;
    }

    pub(crate) fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    pub(crate) fn render(&self, frame_data: &FrameData, store: &ComponentStore) {}
}
