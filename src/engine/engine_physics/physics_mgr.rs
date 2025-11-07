pub struct PhysicsManager {
    components: Vec<u64>,
}



impl PhysicsManager {
    pub(crate) fn new() -> Self {
        Self{
            components: Vec::new(),
        }
    }
    pub(crate) fn register_components(&mut self, components: Vec<u64>) {
        self.components = components;
    }
}
