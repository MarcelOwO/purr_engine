use crate::engine_entities::entitie::Entity;

pub(crate) struct Renderer {}

impl Renderer {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub(crate) fn render(&self, render_queue: Vec<&mut Box<dyn Entity>>, cameras: Vec<&mut Box<dyn Entity>>) {}
}

