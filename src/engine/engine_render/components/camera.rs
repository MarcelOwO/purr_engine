use crate::engine_core::frame_data::FrameData;
use crate::engine_entities::actors::Actor;
use crate::engine_entities::component::Component;
use crate::engine_entities::component_store::ComponentStore;
use crate::engine_entities::entity_type::EntityType;

pub struct Camera<'a> {
    entity_type: EntityType,
    actor: &'a Actor,
    id: u64,
}

impl<'a> Camera<'a> {
    pub fn new(actor: &'a Actor) -> Self {
        Self {
            entity_type: EntityType::Camera,
            actor,
            id: 0,
        }
    }
}

impl<'a> Component for Camera<'a> {
    fn init(&mut self) {}

    fn update(&mut self, frame_data: &FrameData) {}

    fn get_type(&self) -> &EntityType {
        &self.entity_type
    }

    fn get_actor(&self) -> &Actor {
        &self.actor
    }

    fn get_id(&self) -> u64 {
        self.id
    }
}

