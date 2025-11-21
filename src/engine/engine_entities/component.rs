use crate::engine_core::frame_data::FrameData;
use crate::engine_entities::actors::Actor;
use crate::engine_entities::entity_type::ComponentType;

pub trait Component: Send + Sync {
    fn get_type(&self) -> &ComponentType;
    fn get_actor_id(&self) -> u64;
    fn get_id(&self) -> u64;
}

pub trait UpdateComponent: Send + Sync + Component {
    fn update(&mut self, frame_data: &FrameData);
}

pub trait RenderComponent: Send + Sync + Component {
    fn render(&self);
}
