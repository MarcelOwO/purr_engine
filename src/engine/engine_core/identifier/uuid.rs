// simple uuid system so I can reference objects by id
// 0 id is reserved for null references
use std::sync::atomic::{AtomicU64, Ordering};

static UUID_COUNTER: AtomicU64 = AtomicU64::new(1);

pub(crate) fn get_uuid() -> u64 {
    UUID_COUNTER.fetch_add(1, Ordering::Relaxed)
}
