pub(crate) struct FrameData {
    pub(crate) delta_time: f32,
}
impl FrameData {
    pub(crate) fn new() -> Self {
        Self { delta_time: 0.0 }
    }
}
