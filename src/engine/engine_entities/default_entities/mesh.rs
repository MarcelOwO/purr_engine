use crate::engine_entities::actors::Actor;
use crate::engine_entities::entitie::Entity;
use crate::engine_entities::entity_type::EntityType;
struct Mesh<'a> {
    entity_type: EntityType,
    actor:  &'a Actor,
}

impl<'a> Mesh<'a>{
    fn new(actor: &'a Actor)->Self{
       Self{entity_type: EntityType::Mesh,  actor }
    }

}

impl<'a> Entity for Mesh<'a> {
    fn init(&mut self) {

    }

    fn update(&mut self) {}

    fn get_type(&self) -> &EntityType {
        &self.entity_type
    }
    fn get_actor(&self) ->&Actor {
        &self.actor
    }
}