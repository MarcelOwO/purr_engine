use crate::engine_core::frame_data::FrameData;
use crate::engine_entities::actors::Actor;
use crate::engine_entities::component::Component;
use crate::engine_entities::component_store::ComponentStore;
use crate::engine_entities::entity_type::EntityType;

pub struct Light {}

impl Light {
    fn new() -> Self {
        Self {}
    }
}

impl Component for Light {
    fn init(&mut self) {
        todo!()
    }

    fn update(&mut self, frame_data: &FrameData) {
        todo!()
    }

    fn get_type(&self) -> &EntityType {
        todo!()
    }

    fn get_actor(&self) -> &Actor {
        todo!()
    }

    fn get_id(&self) -> u64 {
        todo!()
    }
}

