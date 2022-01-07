#[cfg(not(test))]
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static HISTORY_ID: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(test))]
pub trait ParagraphPropertyChangeId {
    fn generate(&self) -> String {
        use std::sync::atomic::Ordering;

        let id = HISTORY_ID.load(Ordering::Relaxed);
        HISTORY_ID.store(id + 1, Ordering::Relaxed);
        format!("{}", id)
    }
}

#[cfg(test)]
pub trait ParagraphPropertyChangeId {
    fn generate(&self) -> &str {
        "123"
    }
}
