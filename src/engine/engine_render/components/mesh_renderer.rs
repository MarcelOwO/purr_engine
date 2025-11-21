use crate::engine_core::frame_data::FrameData;
use crate::engine_entities::actors::Actor;
use crate::engine_entities::component::{Component, RenderComponent};
use crate::engine_entities::component_store::ComponentStore;
use crate::engine_entities::entity_type::ComponentType;
use std::cmp::PartialEq;
use std::mem;

pub(crate) struct MeshRenderer {
    settings: MeshRendererSettings,
    entity_type: ComponentType,
    own_id: u64,
    actor_id: u64,
}

#[derive(Default)]
pub struct MeshRendererSettings {}

impl MeshRenderer {
    fn new(mut f: impl FnMut(&mut MeshRendererSettings)) -> Self {
        let mut settings = MeshRendererSettings::default();
        f(&mut settings);

        Self {
            settings,
            entity_type: ComponentType::MeshRenderer,
            own_id: 0,
            actor_id: 0,
        }
    }
}

impl PartialEq for ComponentType {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl Component for MeshRenderer {
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

impl RenderComponent for MeshRenderer{
    fn render(&self) {
    }
}