use crate::{
    engine_assets::asset_manager::AssetManager,
    engine_audio::audio_mgr::AudioManager,
    engine_core::{
        frame_data::FrameData,
        logging::{console_logger::ConsoleLogger, logger::Logger},
        module::Module,
        settings::Settings,
    },
    engine_entities::{component_store::ComponentStore, entity_type::EntityType},
    engine_input::input_mgr::InputManager,
    engine_physics::physics_mgr::PhysicsManager,
    engine_render::renderer::Renderer,
    engine_scene::scene_mgr::SceneManager,
    engine_window::window_mgr::WindowMgr,
};

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

struct LogicSystem {
    is_running: bool,
    logger: Arc<dyn Logger>,
    scene_manager: SceneManager,
    asset_manager: AssetManager,
    renderer: Renderer,
    store: ComponentStore,
    physics_manager: PhysicsManager,
    audio_manager: AudioManager,
    input_manager: InputManager,
    time: SystemTime,
    frame_data: FrameData,
}

impl LogicSystem {
    fn new(logger: Arc<dyn Logger>) -> Self {
        Self {
            is_running: true,
            logger: logger.clone(),
            scene_manager: SceneManager::new(logger.clone()),
            asset_manager: AssetManager::new(),
            renderer: Renderer::new(),
            store: ComponentStore::new(),
            physics_manager: PhysicsManager::new(),
            audio_manager: AudioManager::new(),
            input_manager: InputManager::new(),
            time: SystemTime::now(),
            frame_data: FrameData::new(),
        }
    }

    fn update(&mut self) {
        while (self.is_running) {
            self.logger.log("looping...");
            let delta = self.time.elapsed().unwrap().as_secs_f32();
            self.frame_data.delta_time = delta;

            self.physics_manager.update(&self.frame_data, &self.store);
            self.audio_manager.update(&self.frame_data, &self.store);
            self.renderer.render(&self.frame_data, &self.store);

            self.scene_manager.mutate(|scene| {
                scene.mutate(|actor| {
                    for component_id in actor.get_components() {
                        let component = self.store.get_component(*component_id).unwrap();
                        component.update(&self.frame_data);
                    }
                });
            });

            let frame_time = 0.16666667;

            if (delta < frame_time) {
                sleep(Duration::from_secs_f32(frame_time - delta));
            }

            self.time = SystemTime::now();
        }
    }
    fn init(&mut self) {
        let mut renderable: Vec<u64> = vec![];
        let mut physicsable: Vec<u64> = vec![];
        let mut audioable: Vec<u64> = vec![];

        self.scene_manager.mutate(|scene| {
            scene.mutate(|actor| {
                actor.mutate(|component_id| {
                    let component = self.store.get_component(component_id).unwrap();
                    component.init();
                    match component.get_type() {
                        EntityType::Camera => renderable.push(component_id),
                        EntityType::Mesh => renderable.push(component_id),
                        EntityType::MeshRenderer => renderable.push(component_id),
                        EntityType::PointLight => renderable.push(component_id),
                        EntityType::AudioSource => audioable.push(component_id),
                    }
                });
            });
        });

        self.physics_manager.register_components(physicsable);
        self.audio_manager.register_components(audioable);
        self.renderer.register_components(renderable);
    }
}

pub struct App {
    settings: Settings,
    logger: Arc<dyn Logger>,
    window: WindowMgr,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        let logger = Arc::new(ConsoleLogger::new());
        Self {
            settings,
            logger: logger.clone(),
            window: WindowMgr::new(logger.clone()),
        }
    }

    pub(crate) fn register_modules(&mut self, modules: Vec<Box<dyn Module>>) -> &Self {
        self.logger.log("OwO");
        //implement this later, still need to think how I should do this
        self
    }

    //init all components in all scenes and register them to corresponding systems
    pub fn init(mut self) {
        //for now hack to get this to kinda work
        self.logger.log("Init app");
        let logic = Arc::new(Mutex::new(LogicSystem::new(self.logger.clone())));
        let logic_clone = Arc::clone(&logic);

        thread::spawn(move || {
            let mut logic = logic_clone.lock().unwrap();
            logic.logger.log("Spawned logic thread");
            logic.init();
            logic.update();
        });

        self.logger.log("Starting window");
        // none exititng call to start window and event loop
        self.window.run();
    }
}
