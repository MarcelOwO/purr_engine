pub struct Asset {
    path: String,
}

impl Asset {
    pub fn new(path: String) -> Self {
        Self{path}
    }
    fn load(&self) {}
}
