#[cfg(not(test))]
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static HYPERLINK_ID: AtomicUsize = AtomicUsize::new(1);

#[cfg(not(test))]
pub fn generate_hyperlink_id() -> usize {
    use std::sync::atomic::Ordering;

    let id = HYPERLINK_ID.load(Ordering::Relaxed);
    HYPERLINK_ID.store(id.wrapping_add(1), Ordering::Relaxed);
    id
}

#[cfg(test)]
pub fn generate_hyperlink_id() -> usize {
    123
}

pub fn create_hyperlink_rid(id: usize) -> String {
    format!("rIdHyperlink{}", id)
}
