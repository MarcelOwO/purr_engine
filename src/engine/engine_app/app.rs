use crate::engine_assets::asset_manager::AssetManager;
use crate::engine_audio::audio_mgr::AudioManager;
use crate::engine_core::identifier::uuid::UUID;
use crate::engine_core::logging::console_logger::ConsoleLogger;
use crate::engine_core::logging::logger::Logger;
use crate::engine_core::module::Module;
use crate::engine_core::settings::Settings;
use crate::engine_entities::component::Component;
use crate::engine_entities::component_store::ComponentStore;
use crate::engine_entities::entity_type::EntityType;
use crate::engine_input::input_mgr::InputManager;
use crate::engine_physics::physics_mgr::PhysicsManager;
use crate::engine_render::renderer::Renderer;
use crate::engine_scene::scene_mgr::SceneManager;
use crate::engine_window::window_mgr::WindowMgr;

pub struct App {
    settings: Settings,
    logger: Box<dyn Logger>,
    scene_manager: SceneManager,
    window: WindowMgr,
    asset_manager: AssetManager,
    renderer: Renderer,
    id: UUID,
    store: ComponentStore,
    physics_manager: PhysicsManager,
    audio_manager: AudioManager,
    input_manager: InputManager,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            logger: Box::new(ConsoleLogger::new()),
            scene_manager: SceneManager::new(),
            window: WindowMgr::new(),
            asset_manager: AssetManager::new(),
            renderer: Renderer::new(),
            id: UUID::new(),
            store: ComponentStore::new(),
            physics_manager: PhysicsManager::new(),
            audio_manager: AudioManager::new(),
            input_manager: InputManager::new(),
        }
    }

    pub(crate) fn register_modules(&self, modules: Vec<Box<dyn Module>>) -> &Self {
        for mut module in modules {
            module.setup();
            let mut scene = module.get_scene();
        }
        self
    }

    //init all components in all scenes and register them to corresponding systems
    pub fn init(&mut self) {
        let mut renderable: Vec<u64> = vec![];
        let mut physicsable: Vec<u64> = vec![];
        let mut audioable: Vec<u64> = vec![];

        self.scene_manager.mutate(|scene| {
            scene.mutate(|actor| {
                for component_id in &mut actor.entities {
                    let component = self.store.get_component(*component_id).unwrap();
                    component.init();

                    match (component.get_type()) {
                        EntityType::Camera => renderable.push(*component_id),
                        EntityType::Mesh => renderable.push(*component_id),
                        EntityType::MeshRenderer => renderable.push(*component_id),
                    }
                }
            });
        });
        self.physics_manager.register_components(physicsable);
        self.audio_manager.register_components(audioable);
        self.renderer.register_components(renderable);
    }

    pub fn run(&mut self) {
        loop {
            self.input_manager.update();
            self.physics_manager.update();
            self.audio_manager.update();
            self.renderer.render();

            self.scene_manager.mutate(|scene| {
                scene.mutate(|actor| {
                    for component_id in &mut actor.entities {
                        let component = self.store.get_component(*component_id).unwrap();
                        component.update();
                    }
                });
            });
        }
    }
}
