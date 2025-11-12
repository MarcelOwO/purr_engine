use crate::engine_core::{identifier::uuid::get_uuid, logging::logger::Logger};
use crate::engine_scene::scene::Scene;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) struct SceneManager {
    scenes: HashMap<u64, Scene>,
    active_scenes: Vec<u64>,
    logger: Arc<dyn Logger>,
}

impl SceneManager {
    pub(crate) fn get_scene(&mut self, id: u64) -> &mut Scene {
        self.scenes.get_mut(&id).unwrap()
    }
}

impl SceneManager {
    pub(crate) fn new(logger: Arc<dyn Logger>) -> Self {
        Self {
            scenes: HashMap::new(),
            active_scenes: Vec::new(),
            logger,
        }
    }

    pub(crate) fn mutate(&mut self, mut f: impl FnMut(&mut Scene)) {
        for id in self.active_scenes.iter().copied() {
            if let Some(scene) = self.scenes.get_mut(&id) {
                f(scene);
            }
        }
    }

    pub(crate) fn add_scene(&mut self, scene: Scene) {
        self.logger.log("Adding new scene");
        let id = get_uuid();
        self.scenes.insert(id, scene);
    }
    fn remove_scene(&mut self, id: u64) {
        self.logger.log("removing  Scene");
        self.scenes.remove(&id);
        self.set_scene_inactive(id);
    }

    fn set_scene_active(&mut self, id: u64) {
        if !self.active_scenes.contains(&id) {
            self.active_scenes.push(id);
        }
    }

    fn set_scene_inactive(&mut self, id: u64) {
        self.active_scenes.retain(|&scene_id| scene_id != id);
    }
}
