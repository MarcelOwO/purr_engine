use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

struct WindowWinit {
    window: Option<Window>,
}

impl WindowWinit {

}

impl ApplicationHandler for WindowWinit{
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        todo!()
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: ()) {
        todo!()
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        todo!()
    }

    fn device_event(&mut self, event_loop: &ActiveEventLoop, device_id: DeviceId, event: DeviceEvent) {
        todo!()
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }

    fn memory_warning(&mut self, event_loop: &ActiveEventLoop) {
        todo!()
    }
}


