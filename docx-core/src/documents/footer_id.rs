/*
#[cfg(not(test))]
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static FOOTER_ID: AtomicUsize = AtomicUsize::new(1);

#[cfg(not(test))]
pub fn generate_pic_id() -> usize {
    use std::sync::atomic::Ordering;

    let id = FOOTER_ID.load(Ordering::Relaxed);
    FOOTER_ID.store(id.wrapping_add(1), Ordering::Relaxed);
    id
}

#[cfg(test)]
pub fn generate_footer_id() -> usize {
    123
}
*/

pub fn create_footer_rid(id: usize) -> String {
    format!("rIdFooter{}", id)
}
