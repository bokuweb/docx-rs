use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct WebExtension(docx_rs::WebExtension);

#[wasm_bindgen(js_name = createWebExtension)]
pub fn create_web_extension(
    id: &str,
    reference_id: &str,
    version: &str,
    store: &str,
    store_type: &str,
) -> WebExtension {
    WebExtension(docx_rs::WebExtension::new(
        id,
        reference_id,
        version,
        store,
        store_type,
    ))
}

impl WebExtension {
    pub fn take(self) -> docx_rs::WebExtension {
        self.0
    }
}

#[wasm_bindgen]
impl WebExtension {
    pub fn property(mut self, name: &str, value: &str) -> Self {
        self.0 = self.0.property(name, value);
        self
    }
}
