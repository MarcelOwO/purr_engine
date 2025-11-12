/// Window implementation using winit
///
/// Absolute mess since it required a seperate runner? So 2 struct... i guess
use crate::{
    engine_core::logging::logger::Logger,
    engine_window::{
        window_error::{Result, WindowError},
        window_impl::WindowImpl,
    },
};
use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::Size,
    event::{DeviceEvent, DeviceId, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopBuilder},
    platform::{pump_events::EventLoopExtPumpEvents, run_on_demand::EventLoopExtRunOnDemand},
    window::{Window, WindowAttributes, WindowId},
};

pub(crate) struct WindowWinit {
    logger: Arc<dyn Logger>,
}

impl WindowWinit {
    pub(crate) fn new(logger: Arc<dyn Logger>) -> Self {
        logger.log("create winit subsystem");
        Self {
            logger: logger.clone(),
        }
    }
}

impl WindowImpl<()> for WindowWinit {
    fn run(&mut self) -> Result<()> {
        self.logger.log("Starting window event loop");

        let event_loop = EventLoop::new().unwrap();

        let mut app = WinitApp::new(self.logger.clone());

        event_loop.run_app(&mut app);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct WinitApp {
    window: Option<Window>,
    logger: Arc<dyn Logger>,
}

impl WinitApp {
    fn new(logger: Arc<dyn Logger>) -> Self {
        logger.log("Creating winit runner");
        Self {
            window: None,
            logger,
        }
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.logger.log("Resuned window and created window");
        let mut attributes = WindowAttributes::default();

        attributes.fullscreen = Some(winit::window::Fullscreen::Borderless(None));
        attributes.title = "OwO".to_string();

        attributes.inner_size = Some(Size::Logical(winit::dpi::LogicalSize {
            width: 800.0,
            height: 600.0,
        }));

        let window = event_loop.create_window(attributes).unwrap();

        window.set_visible(true);
        window.set_cursor_visible(true);
        self.window = Some(window)
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.logger.log("The close button was pressed; stopping");
                event_loop.exit();
            }
            _ => {
                let event_name = format!("{event:?}");
                self.logger.log(event_name.as_str())
            }
        }
    }
}
