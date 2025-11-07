pub(crate) struct AudioManager {
    components: Vec<u64>,
}

impl AudioManager {
    pub(crate) fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    pub(crate) fn register_components(&mut self, components: Vec<u64>) {
        self.components = components;
    }
}
