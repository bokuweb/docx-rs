use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct Numberings {}

impl Numberings {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Numberings {
    fn default() -> Self {
        Self {}
    }
}

impl BuildXML for Numberings {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new().declaration(Some(true)).open_numbering();
        b = add_default_numbering(b);
        b.close().build()
    }
}

fn add_default_numbering(b: XMLBuilder) -> XMLBuilder {
    let mut b = b.open_abstract_num("0");
    b = b.add_child(
        &Level::new(
            0,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%1."),
            LevelJc::new("left"),
        )
        .indent(420, Some(SpecialIndentType::Hanging(420))),
    );
    b = b.add_child(
        &Level::new(
            1,
            Start::new(1),
            NumberFormat::new("decimal"),
            LevelText::new("%2."),
            LevelJc::new("left"),
        )
        .indent(840, Some(SpecialIndentType::Hanging(420))),
    );
    b.close()
}
