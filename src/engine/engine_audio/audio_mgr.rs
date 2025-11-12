use crate::engine_core::frame_data::FrameData;
use crate::engine_entities::component_store::ComponentStore;

pub(crate) struct AudioManager {
    components: Vec<u64>,
}

impl AudioManager {
    pub(crate) fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    pub(crate) fn register_components(&mut self, components: Vec<u64>) {
        self.components = components;
    }
    pub(crate) fn update(&self, frame_data: &FrameData, store: &ComponentStore) {}
}
