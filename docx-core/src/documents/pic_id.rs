#[cfg(not(test))]
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static PIC_ID: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(test))]
pub fn generate_pic_id() -> String {
    use std::sync::atomic::Ordering;

    let id = PIC_ID.load(Ordering::Relaxed);
    PIC_ID.store(id + 1, Ordering::Relaxed);
    format!("{}", id)
}

#[cfg(test)]
pub fn generate_pic_id() -> String {
    "123".to_owned()
}
