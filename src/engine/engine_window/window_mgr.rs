use winit::window::Window;
use crate::engine_window::window;
use std::collections::HashMap;
pub(crate) struct WindowMgr {
    windows: HashMap<String, Window>,
}
impl WindowMgr {
    pub(crate) fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

}
