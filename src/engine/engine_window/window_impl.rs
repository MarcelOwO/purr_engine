use crate::engine_window::window_error::Result;

pub(crate) trait WindowImpl<T> {
    fn run(&mut self);
    fn init(&mut self);
}
