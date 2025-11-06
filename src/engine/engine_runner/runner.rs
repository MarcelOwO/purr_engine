use crate::engine_assets::asset_manager::AssetManager;
use crate::engine_core::logging::console_logger::ConsoleLogger;
use crate::engine_core::logging::logger::Logger;
use crate::engine_core::settings::Settings;
use crate::engine_entities::entitie;
use crate::engine_entities::entity_type::EntityType;
use crate::engine_render::renderer::Renderer;
use crate::engine_scene::scene::Scene;
use crate::engine_window::window_mgr::WindowMgr;
use entitie::Entity;

pub struct Runner {
    settings: Settings,
    logger: Box<dyn Logger>,
    scenes: Vec<Box<dyn Scene>>,
    window: WindowMgr,
    asset_manager: AssetManager,
    renderer: Renderer,
}

impl Runner {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            logger: Box::new(ConsoleLogger::new()),
            scenes: Vec::new(),
            window: WindowMgr::new(),
            asset_manager: AssetManager::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn add_scene(&mut self, scene: Box<dyn Scene>) {
        self.scenes.push(scene);
    }

    pub fn init(&mut self) {
        for scene in &mut self.scenes {
            for actor in scene.get_actors() {
                for entity in &mut actor.entities {
                    entity.init();
                }
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            let mut render_queue = Vec::new();
            let mut cameras = Vec::new();

            for scene in &mut self.scenes {
                for actor in scene.get_actors() {
                    for entity in &mut actor.entities {
                        entity.update();
                        match (entity.get_type()) {
                            EntityType::Camera => {
                                cameras.push(entity);
                            }
                            EntityType::Mesh => {}
                            EntityType::MeshRenderer => {
                                 render_queue.push(entity);
                            }
                        }
                    }
                }
            }
            self.renderer.render(render_queue, cameras);
        }
    }
    fn process_entities(&mut self, entity: &mut Box<dyn Entity>) {}
}
