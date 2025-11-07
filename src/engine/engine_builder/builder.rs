use crate::engine_app::app::App;
use crate::engine_core::module::Module;
use crate::engine_core::settings::Settings;

//scuffed builder pattern for app
pub struct Builder {
    app: Option<App>,
    settings: Option<Settings>,
    modules: Vec<Box<dyn Module>>,
}

impl Builder {
    //create empty builder
    pub fn new() -> Self {
        Self {
            app: None,
            settings: None,
            modules: Vec::new(),
        }
    }

    //configure settings
    pub fn configure(&mut self, mut f: impl FnMut(&mut Settings)) -> &mut Self {
        let mut settings = Settings::default();
        f(&mut settings);
        self.settings = Some(settings);
        self
    }

    pub fn register_module<T: Module + Default + 'static>(&mut self) -> &mut Self {
        self.modules.push(Box::new(T::default()));
        self
    }

    //build and optimize stuff
    pub fn build(&mut self) -> &mut Self {
        let app = App::new(self.settings.take().unwrap());
        app.register_modules(self.modules.drain(..).collect());
        self.app = Some(app);
        self
    }

    //start and run the app
    pub fn run(&mut self) {
        let mut app = self.app.take().unwrap();
        app.init();
        app.run();
    }
}
