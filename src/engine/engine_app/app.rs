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

    pub(crate) fn init(&mut self) {
        let mut renderable: Vec<u64> = vec![];
        let mut physicsable: Vec<u64> = vec![];
        let mut audioable: Vec<u64> = vec![];

        self.scene_manager.mut_active_scenes(|scene| {
            scene.mut_actors(|actor_id| {
                self.component_store.mut_components_of_actor(
                    &actor_id,
                    |component| match component.get_type() {
                        ComponentType::Camera => renderable.push(component.get_id()),
                        ComponentType::Mesh => renderable.push(component.get_id()),
                        ComponentType::MeshRenderer => renderable.push(component.get_id()),
                        ComponentType::PointLight => renderable.push(component.get_id()),
                        ComponentType::AudioSource => audioable.push(component.get_id()),
                    },
                );
            });
        });

        self.physics_manager.register_components(physicsable);
        self.audio_manager.register_components(audioable);
        self.renderer.register_components(renderable);
    }

    pub(crate) fn update(&mut self) {
        let delta = self.time.elapsed().unwrap().as_secs_f32();
        self.frame_data.delta_time = delta;

        self.physics_manager
            .update(&self.frame_data, &self.component_store);
        self.audio_manager
            .update(&self.frame_data, &self.component_store);
        self.renderer
            .render(&self.frame_data, &self.component_store);

        self.scene_manager.mut_active_scenes(|scene| {
            scene.mut_actors(|actor_id| {
                self.component_store
                    .mut_components_of_actor(&actor_id, |component| {});
            });
        });

        let frame_time = 0.16666667;

        if delta < frame_time {
            sleep(Duration::from_secs_f32(frame_time - delta));
        }

        self.time = SystemTime::now();
    }

    pub(crate) fn create_scene(&mut self) -> u64 {
        self.scene_manager.create_scene()
    }

    pub(crate) fn create_actor(
        &mut self,
        scene_id: u64,
        mut f: impl FnMut(&mut Actor),
    ) -> &mut Self {
        let id = self.component_store.create_actor(|actor| {
            f(actor);
        });
        let scene = self.scene_manager.get_scene(scene_id);

        scene.add_actor(id);

        self
    }

    pub(crate) fn add_component<T: Component + Default + 'static>(
        &mut self,
        actor_id: u64,
        mut f: impl FnMut(&mut Box<T>),
    ) -> &mut Self {
        let mut id = self
            .component_store
            .create_component::<T>(actor_id, |component| {
                f(component);
            });
        self
    }

    pub(crate) fn update_setting(&mut self, settings: Settings) {
        self.settings = settings;
    }

    pub(crate) fn mut_asset_manager(&mut self, mut f: impl FnMut(&mut AssetManager)) {
        f(&mut self.asset_manager);
    }
}
