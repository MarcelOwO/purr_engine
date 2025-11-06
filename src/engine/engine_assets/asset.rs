pub struct Asset {
    path: String,
}

impl Asset {
    fn new(path: String) -> Self {
        Self{path}
    }
    fn load(&self) {}
}
