use crate::engine_entities::actors::Actor;
use crate::engine_entities::component_store::ComponentStore;
use crate::engine_entities::entity_type::EntityType;

pub trait Component {
    fn init(&mut self);
    fn update(&mut self);
    fn get_type(&self) -> &EntityType;
    fn get_actor(&self) -> &Actor;
    fn get_id(&self) -> u64;
}
