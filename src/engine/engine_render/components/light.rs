use crate::engine_entities::{component::Component, entity_type::ComponentType};
pub struct Light {
    settings: LightSettings,
    entity_type: ComponentType,
    own_id: u64,
    actor_id: u64,
}

#[derive(Default)]
pub struct LightSettings {
    pub strength: f32,
}

impl Light {
    fn new(mut f: impl FnMut(&mut LightSettings)) -> Self {
        let mut settings = LightSettings::default();
        f(&mut settings);
        Self {
            settings,
            entity_type: ComponentType::PointLight,
            own_id: 0,
            actor_id: 0,
        }
    }
}

impl Component for Light {
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
