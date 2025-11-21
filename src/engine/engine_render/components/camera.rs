use crate::{
    engine_core::frame_data::FrameData,
    engine_entities::{
        component::{Component, RenderComponent},
        entity_type::ComponentType,
    },
};

pub struct Camera {
    settings: CameraSettings,
    entity_type: ComponentType,
    own_id: u64,
    actor_id: u64,
}

#[derive(Default)]
pub struct CameraSettings {
    pub aperture: f32,
    // shutter_speed:f32,
    pub orthographic: bool,
}

impl Camera {
    pub fn new(mut f: impl FnMut(&mut CameraSettings)) -> Self {
        let mut settings = CameraSettings::default();

        f(&mut settings);

        Self {
            entity_type: ComponentType::Camera,
            settings,
            own_id: 0,
            actor_id: 0,
        }
    }
}

impl Component for Camera {
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

impl RenderComponent for Camera {
    fn render(&self) {}
}
