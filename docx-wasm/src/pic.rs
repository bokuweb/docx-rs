use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Pic(docx_rs::Pic);

#[wasm_bindgen(js_name = createPic)]
pub fn create_pic(buf: &[u8]) -> Pic {
    Pic(docx_rs::Pic::new(buf))
}

#[wasm_bindgen]
impl Pic {
    // unit is emu
    pub fn size(mut self, w_emu: u32, h_emu: u32) -> Self {
        self.0 = self.0.size(w_emu, h_emu);
        self
    }

    // unit is deg
    pub fn rotate(mut self, deg: u16) -> Self {
        self.0 = self.0.rotate(deg);
        self
    }

    pub fn floating(mut self) -> Pic {
        self.0 = self.0.floating();
        self
    }

    pub fn offset_x(mut self, x: i32) -> Pic {
        self.0 = self.0.offset_x(x);
        self
    }

    pub fn offset_y(mut self, y: i32) -> Pic {
        self.0 = self.0.offset_y(y);
        self
    }

    // TODO: add other fns...
}

impl Pic {
    pub fn take(self) -> docx_rs::Pic {
        self.0
    }
}
