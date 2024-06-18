#[cfg(not(test))]
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static FOOTNOTE_ID: AtomicUsize = AtomicUsize::new(1);

#[cfg(not(test))]
pub fn generate_footnote_id() -> usize {
    use std::sync::atomic::Ordering;

    let id = FOOTNOTE_ID.load(Ordering::Relaxed);
    FOOTNOTE_ID.store(id.wrapping_add(1), Ordering::Relaxed);
    id
}

#[cfg(test)]
pub fn generate_footnote_id() -> usize {
    1
}
