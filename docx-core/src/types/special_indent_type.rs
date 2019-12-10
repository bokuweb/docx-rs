use wasm_bindgen::prelude::*;

// INFO: wasm-bindgen only allow c-style enum for now
//       Please convert typescript type to following type.
#[derive(Copy, Clone, Debug)]
pub enum SpecialIndentType {
    FirstLine(usize),
    Hanging(usize),
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum SpecialIndentKind {
    FirstLine,
    Hanging,
}
