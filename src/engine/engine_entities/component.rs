use crate::engine_core::frame_data::FrameData;
use crate::engine_entities::actors::Actor;
use crate::engine_entities::entity_type::EntityType;

pub trait Component: Send + Sync {
    fn init(&mut self);
    fn update(&mut self, frame_data: &FrameData);
    fn get_type(&self) -> &EntityType;
    fn get_actor(&self) -> &Actor;
    fn get_id(&self) -> u64;
}
