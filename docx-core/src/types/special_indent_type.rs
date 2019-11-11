// INFO: wasm-bindgen allow c-style enum for now
//       Please convert typescript type to following type.
#[derive(Copy, Clone, Debug)]
pub enum SpecialIndentType {
    FirstLine(usize),
    Hanging(usize),
}
