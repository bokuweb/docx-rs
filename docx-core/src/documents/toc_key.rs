#[cfg(not(test))]
use std::sync::atomic::AtomicUsize;
#[cfg(not(test))]
static TOC_KEY: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(test))]
pub trait TocKeyGenerator {
    fn generate() -> String {
        use std::sync::atomic::Ordering;

        let id = TOC_KEY.load(Ordering::Relaxed);
        TOC_KEY.store(id + 1, Ordering::Relaxed);
        format!("_Toc{:08}", id)
    }
}

pub struct TocKey {}

impl TocKeyGenerator for TocKey {}

#[cfg(test)]
pub trait TocKeyGenerator {
    fn generate() -> String {
        "_Toc00000000".to_string()
    }
}
