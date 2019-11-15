#[allow(unused)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(dead_code)]
static HISTORY_ID: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(test))]
pub trait HistoryId {
    fn generate(&self) -> String {
        let id = HISTORY_ID.load(Ordering::Relaxed);
        HISTORY_ID.store(id + 1, Ordering::Relaxed);
        format!("{}", id)
    }
}

#[cfg(test)]
pub trait HistoryId {
    fn generate(&self) -> &str {
        "123"
    }
}
