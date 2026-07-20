use wasm_bindgen::prelude::*;

fn is_emf(path: &str, buf: &[u8]) -> bool {
    if path.to_ascii_lowercase().ends_with(".emf") {
        return true;
    }

    buf.len() >= 44
        && buf[0..4] == [0x01, 0x00, 0x00, 0x00]
        && buf[40..44] == [0x20, 0x45, 0x4D, 0x46]
}

fn convert_emf_to_svg(buf: &[u8]) -> Option<Vec<u8>> {
    let wmf_player = wmf_core::converter::SVGPlayer::new();
    let emf_player = emf_core::converter::SVGPlayer::new();
    let converter = emf_core::converter::EMFConverter::new(buf, emf_player, wmf_player);
    converter.run().ok()
}

fn add_emf_previews(docx: &mut docx_rs::Docx) {
    for (_, path, original, preview) in &mut docx.images {
        if is_emf(path, &original.0) {
            if let Some(svg) = convert_emf_to_svg(&original.0) {
                preview.0 = svg;
            }
        }
    }
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn readDocx(buf: &[u8]) -> Result<String, JsValue> {
    let mut d = docx_rs::read_docx(buf);
    match d {
        Ok(ref mut d) => {
            add_emf_previews(d);
            Ok(d.json())
        }
        Err(e) => Err(e.to_string().into()),
    }
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn readXML(xml: &str) -> Result<String, JsValue> {
    let mut d = docx_rs::read_xml(xml);
    match d {
        Ok(ref mut d) => Ok(d.json()),
        Err(e) => Err(e.to_string().into()),
    }
}
