#[cfg(not(test))]
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static BOOKMARK_ID: AtomicUsize = AtomicUsize::new(1);

#[cfg(not(test))]
pub fn generate_bookmark_id() -> usize {
    use std::sync::atomic::Ordering;

    let id = BOOKMARK_ID.load(Ordering::Relaxed);
    BOOKMARK_ID.store(id.wrapping_add(1), Ordering::Relaxed);
    id
}

#[cfg(not(test))]
pub fn reset_bookmark_id() {
    use std::sync::atomic::Ordering;

    BOOKMARK_ID.load(Ordering::Relaxed);
    BOOKMARK_ID.store(1, Ordering::Relaxed);
}

#[cfg(test)]
pub fn generate_bookmark_id() -> usize {
    1
}

#[cfg(test)]
pub fn reset_bookmark_id() {
    // NOP
}
