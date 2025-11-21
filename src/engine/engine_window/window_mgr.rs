use crate::{
    engine_core::logging::logger::Logger,
    engine_window::{
        window_error::{Result, WindowError},
        window_impl::WindowImpl,
        winit::window_winit::WindowWinit,
    },
};
use std::rc::Rc;

pub(crate) struct WindowMgr {
    window: Box<dyn WindowImpl<()>>,
    logger: Rc<dyn Logger>,
}

impl WindowMgr {
    pub(crate) fn new(logger: Rc<dyn Logger>) -> Self {
        logger.log("Creating WindowManager");
        Self {
            window: Box::new(WindowWinit::new(logger.clone())),
            logger,
        }
    }
    pub(crate) fn init(&mut self) {
        self.window.init();
    }

    pub(crate) fn run(&mut self) {
        self.logger.log("Starting window loop");
        self.window.run()
    }
}
