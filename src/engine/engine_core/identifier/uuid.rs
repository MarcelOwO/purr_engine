

// simple uuid system so I can reference objects by id
// 0 id is reserved for null references
pub(crate) struct UUID {
    counter: u64,
}

impl UUID {
    pub(crate) fn new() -> Self {
        Self { counter: 0 }
    }
    pub(crate) fn get(&mut self) -> u64 {
        self.counter += 1;
        self.counter
    }
}
