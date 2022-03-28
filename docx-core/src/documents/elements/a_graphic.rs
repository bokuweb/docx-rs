use super::*;
use serde::Serialize;

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AGraphic {
    pub children: Vec<AGraphicData>,
}

impl AGraphic {
    pub fn new() -> AGraphic {
        Default::default()
    }

    pub fn add_graphic_data(mut self, g: AGraphicData) -> Self {
        self.children.push(g);
        self
    }
}

impl BuildXML for AGraphic {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_graphic("http://schemas.openxmlformats.org/drawingml/2006/main");
        for child in &self.children {
            b = b.add_child(child);
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn test_a_graphic_with_textbox_json() {
        let graphic =
            AGraphic::new().add_graphic_data(
                AGraphicData::new(GraphicDataType::WpShape).add_shape(
                    WpsShape::new().add_text_box(WpsTextBox::new().add_content(
                        TextBoxContent::new().add_paragraph(
                            Paragraph::new().add_run(Run::new().add_text("pattern1")),
                        ),
                    )),
                ),
            );
        assert_eq!(
            serde_json::to_string(&graphic).unwrap(),
            r#"{"children":[{"dataType":"wpShape","children":[{"type":"shape","data":{"children":[{"type":"textbox","data":{"children":[{"children":[{"type":"paragraph","data":{"id":"12345678","children":[{"type":"run","data":{"runProperty":{},"children":[{"type":"text","data":{"preserveSpace":true,"text":"pattern1"}}]}}],"property":{"runProperty":{},"tabs":[]},"hasNumbering":false}}],"has_numbering":false}],"hasNumbering":false}}]}}]}]}"#,
        );
    }
}
