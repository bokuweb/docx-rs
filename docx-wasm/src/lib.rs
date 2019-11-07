extern crate docx_core;

use docx_core::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Hoge {
    pub inner: u32,
    pub fuga: u32,
}

#[wasm_bindgen]
pub fn create(inner: u32) -> Hoge {
    Hoge { inner, fuga: 0 }
}

#[wasm_bindgen]
impl Hoge {
    pub fn add(&mut self) {
        self.fuga = 10;
    }

    pub fn log(&self) {
        alert(&format!("Hello, {:?}!", self));
    }
}
