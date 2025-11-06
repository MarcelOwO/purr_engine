use crate::engine_entities::actors::Actor;
use crate::engine_entities::entitie::Entity;
use crate::engine_entities::entity_type::EntityType;
use std::cmp::PartialEq;
use std::mem;

struct MeshRenderer<'a> {
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

impl<'a> Entity for MeshRenderer<'a> {
    fn init(&mut self) {
        let actor = self.get_actor();
        if (actor
            .entities
            .iter()
            .any(|e| e.get_type() == &EntityType::Mesh))
        {
            return;
        }
        panic!("MeshRenderer entity requires a Mesh entity attached to the same actor.");
    }

    fn update(&mut self) {}

    fn get_type(&self) -> &EntityType {
        &self.entity_type
    }
    fn get_actor(&self) -> &Actor {
        &self.actor
    }
}
