#[cfg(not(test))]
use std::cell::Cell;
#[cfg(not(test))]
thread_local! {
    // Resetting a process-global counter lets one document reuse IDs while
    // another thread is still creating paragraphs. Keep reset scope local;
    // build-time normalization handles IDs from paragraphs merged across threads.
    static PARA_ID: Cell<u32> = const { Cell::new(1) };
}

#[cfg(not(test))]
pub fn generate_para_id() -> String {
    let id = PARA_ID.with(|next| {
        let id = next.get();
        next.set(id.checked_add(1).expect("paragraph ID space exhausted"));
        id
    });
    format!("{id:08x}")
}

#[cfg(not(test))]
pub fn reset_para_id() {
    PARA_ID.set(1);
}

#[cfg(test)]
pub fn generate_para_id() -> String {
    "12345678".to_owned()
}

#[cfg(test)]
pub fn reset_para_id() {
    // NOP
}
