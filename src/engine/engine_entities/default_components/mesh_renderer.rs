use crate::engine_entities::actors::Actor;
use crate::engine_entities::component::Component;
use crate::engine_entities::entity_type::EntityType;
use crate::engine_entities::component_store::ComponentStore;
use std::cmp::PartialEq;
use std::mem;

pub(crate) struct MeshRenderer<'a> {
    entity_type: EntityType,
    actor: &'a Actor,
}

impl<'a> MeshRenderer<'a> {
    fn new(actor: &'a Actor) -> Self {
        Self {
            entity_type: EntityType::MeshRenderer,
            actor,
        }
    }
}

impl PartialEq for EntityType {
    fn eq(& self, other: & Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl<'a> Component for MeshRenderer<'a> {
    fn init(&mut self) {
    }

    fn update(&mut self) {}

    fn get_type(&self) -> &EntityType {
        &self.entity_type
    }
    fn get_actor(&self) -> &Actor {
        &self.actor
    }

    fn get_id(&self) -> u64 {
        todo!()
    }
}
