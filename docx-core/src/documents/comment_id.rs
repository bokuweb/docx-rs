#[allow(unused)]
use std::sync::atomic::{AtomicUsize, Ordering};

#[allow(dead_code)]
static COMMENT_ID: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(test))]
pub trait CommentId {
    fn generate(&self) -> String {
        let id = COMMENT_ID.load(Ordering::Relaxed);
        COMMENT_ID.store(id + 1, Ordering::Relaxed);
        format!("{}", id)
    }
}

#[cfg(test)]
pub trait CommentId {
    fn generate(&self) -> &str {
        "123"
    }
}
