#[cfg(not(test))]
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static PARA_ID: AtomicUsize = AtomicUsize::new(1);

#[cfg(not(test))]
pub fn generate_para_id() -> String {
    use std::sync::atomic::Ordering;

    let id = PARA_ID.load(Ordering::Relaxed);
    PARA_ID.store(id.wrapping_add(1), Ordering::Relaxed);
    format!("{:08x}", id)
}

#[cfg(not(test))]
pub fn reset_para_id() {
    use std::sync::atomic::Ordering;

    PARA_ID.load(Ordering::Relaxed);
    PARA_ID.store(1, Ordering::Relaxed);
}

#[cfg(test)]
pub fn generate_para_id() -> String {
    "12345678".to_owned()
}

#[cfg(test)]
pub fn reset_para_id() {
    // NOP
}
