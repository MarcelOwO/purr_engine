use crate::engine_entities::entity_type::EntityType;
use crate::engine_entities::actors::Actor;
pub trait Entity{
    fn init(&mut self);
    fn update(&mut self);
    fn get_type(&self)->&EntityType;
    fn get_actor(&self)->&Actor;
}