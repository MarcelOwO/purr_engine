use crate::{
    engine_core::identifier::uuid::{self, get_uuid},
    engine_entities::{actors::Actor, component::Component},
};
use std::collections::HashMap;

pub(crate) struct ComponentStore {
    components: HashMap<u64, Box<dyn Component>>,
    actors: HashMap<u64, Actor>,
}

impl ComponentStore {
    pub(crate) fn new() -> Self {
        Self {
            components: HashMap::new(),
            actors: HashMap::new(),
        }
    }

    pub(crate) fn create_component<T: Component + Default + 'static>(
        &mut self,
        actor_id: u64,
        mut f: impl FnMut(&mut Box<T>),
    ) -> u64 {
        let mut component = Box::new(T::default());

        f(&mut component);

        let component_id = get_uuid();

        self.components.insert(component_id, component);

        let actor = self.actors.get_mut(&actor_id).unwrap();
        actor.add_component(component_id);
        component_id
    }

    pub(crate) fn delete_component(&mut self, id: &u64) {
        self.components.remove(id);
    }

    pub(crate) fn get_component(&mut self, id: &u64, mut f: impl FnMut(&mut Box<dyn Component>)) {
        let mut component = self.components.get_mut(id).unwrap();
        f(&mut component);
    }

    pub(crate) fn mut_components_of_actor(
        &mut self,
        id: &u64,
        mut f: impl FnMut(&mut Box<dyn Component>),
    ) {
        let actor = self.actors.get_mut(id).unwrap();

        actor.mut_components(|component_id| {
            let mut component = self.components.get_mut(&component_id).unwrap();
            f(&mut component);
        });
    }

    pub(crate) fn create_actor(&mut self, mut f: impl FnMut(&mut Actor)) -> u64 {
        let mut actor = Actor::new();
        f(&mut actor);
        let id = get_uuid();
        self.actors.insert(id, actor);
        id
    }

    pub(crate) fn delete_actor(&mut self, id: &u64) {
        self.actors.remove(id);
    }

    pub(crate) fn get_actor(&mut self, id: &u64, mut f: impl FnMut(&mut Actor)) {
        let mut actor = self.actors.get_mut(id).unwrap();
        f(&mut actor);
    }
}
