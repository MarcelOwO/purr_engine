use crate::engine_audio::audio_mgr::AudioManager;
use crate::engine_core::frame_data::FrameData;
use crate::engine_entities::component_store::ComponentStore;
use crate::engine_entities::entity_type::ComponentType;
use crate::engine_input::input_mgr::InputManager;
use crate::engine_physics::physics_mgr::PhysicsManager;
use crate::engine_render::renderer::Renderer;
use crate::{
    engine_assets::asset_manager::AssetManager,
    engine_core::{
        logging::{console_logger::ConsoleLogger, logger::Logger},
        settings::Settings,
    },
    engine_entities::{actors::Actor, component::Component},
    engine_scene::scene_mgr::SceneManager,
    engine_window::window_mgr::WindowMgr,
};
use std::rc::Rc;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

pub struct App {
    settings: Settings,
    window: WindowMgr,
    component_store: ComponentStore,
    logger: Rc<dyn Logger>,
    scene_manager: SceneManager,
    asset_manager: AssetManager,
    renderer: Renderer,
    store: ComponentStore,
    physics_manager: PhysicsManager,
    audio_manager: AudioManager,
    input_manager: InputManager,
    time: SystemTime,
    frame_data: FrameData,
    is_running: bool,
}

impl App {
    pub(crate) fn new(settings: Settings) -> Self {
        let logger = Rc::new(ConsoleLogger::new());
        Self {
            settings,
            logger: logger.clone(),
            window: WindowMgr::new(logger.clone()),
            component_store: ComponentStore::new(),
            asset_manager: AssetManager::new(),
            scene_manager: SceneManager::new(logger.clone()),
            renderer: Renderer::new(),
            store: ComponentStore::new(),
            physics_manager: PhysicsManager::new(),
            audio_manager: AudioManager::new(),
            input_manager: InputManager::new(),
            time: SystemTime::now(),
            frame_data: FrameData::new(),
            is_running: true,
        }
    }

    pub(crate) fn run(&mut self) {
        self.init();

        self.window.init();

        while (self.is_running) {
            self.window.run();
            self.update();
        }
    }

    pub(crate) fn update(&mut self) {
        let delta = self.time.elapsed().unwrap().as_secs_f32();
        self.frame_data.delta_time = delta;

        self.physics_manager.update(&self.frame_data, &self.store);
        self.audio_manager.update(&self.frame_data, &self.store);
        self.renderer.render(&self.frame_data, &self.store);

        self.scene_manager.mut_active_scenes(|scene| {
            scene.mut_actors(|actor| {
                actor.mut_components(|component_id| {
                    let component = self.store.get_component(component_id).unwrap();
                });
            });
        });

        let frame_time = 0.16666667;

        if (delta < frame_time) {
            sleep(Duration::from_secs_f32(frame_time - delta));
        }

        self.time = SystemTime::now();
    }

    pub(crate) fn update_setting(&mut self, settings: Settings) {
        self.settings = settings;
    }

    pub(crate) fn create_scene(&mut self) -> u64 {
        self.scene_manager.create_scene()
    }
    pub fn add_actor(&mut self, id: u64, mut f: impl FnMut(&mut Actor, u64)) -> &mut Self {
        self
    }

    pub fn add_component(&mut self, actor_id: u64, component: Box<dyn Component>) -> &mut Self {
        self.component_store.add_component(actor_id, component);
        self
    }

    pub(crate) fn mut_asset_manager(&mut self, mut f: impl FnMut(&mut AssetManager)) {
        f(&mut self.asset_manager);
    }

    pub(crate) fn init(&mut self) {
        let mut renderable: Vec<u64> = vec![];
        let mut physicsable: Vec<u64> = vec![];
        let mut audioable: Vec<u64> = vec![];

        self.scene_manager.mut_active_scenes(|scene| {
            scene.mut_actors(|actor| {
                actor.mut_components(|component_id| {
                    let component = self.store.get_component(component_id).unwrap();

                    match component.get_type() {
                        ComponentType::Camera => renderable.push(component_id),
                        ComponentType::Mesh => renderable.push(component_id),
                        ComponentType::MeshRenderer => renderable.push(component_id),
                        ComponentType::PointLight => renderable.push(component_id),
                        ComponentType::AudioSource => audioable.push(component_id),
                    }
                });
            });
        });

        self.physics_manager.register_components(physicsable);
        self.audio_manager.register_components(audioable);
        self.renderer.register_components(renderable);
    }
}
