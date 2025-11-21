use crate::engine_entities::component::Component;
use crate::engine_entities::entity_type::ComponentType;

struct Mesh {
    asset: String,
    entity_type: ComponentType,
    own_id: u64,
    actor_id: u64,
}

impl Mesh {
    fn new(asset: String) -> Self {
        Self {
            entity_type: ComponentType::Mesh,
            asset,
            own_id: 0,
            actor_id: 0,
        }
    }
}

impl Component for Mesh {
    fn get_type(&self) -> &ComponentType {
        &self.entity_type
    }
    fn get_actor_id(&self) -> u64 {
        self.actor_id
    }
    fn get_id(&self) -> u64 {
        self.own_id
    }
}
