use crate::{
    engine_core::logging::logger::Logger,
    engine_window::{
        window_error::{Result, WindowError},
        window_impl::WindowImpl,
        window_winit::WindowWinit,
    },
};

use std::sync::Arc;

pub(crate) struct WindowMgr {
    window: Box<dyn WindowImpl<()>>,
    logger: Arc<dyn Logger>,
}
impl WindowMgr {
    pub(crate) fn new(logger: Arc<dyn Logger>) -> Self {
        logger.log("Creating WindowManager");
        Self {
            window: Box::new(WindowWinit::new(logger.clone())),
            logger,
        }
    }

    pub(crate) fn run(&mut self) -> Result<()> {
        self.logger.log("Starting window loop");
        self.window.run()
    }
}
