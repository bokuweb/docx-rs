use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct Numberings<'a> {
    numberings: Vec<Numbering<'a>>,
}

impl<'a> Numberings<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_numbering(mut self, n: Numbering<'a>) -> Self {
        self.numberings.push(n);
        self
    }
}

impl<'a> Default for Numberings<'a> {
    fn default() -> Self {
        Self { numberings: vec![] }
    }
}

impl<'a> BuildXML for Numberings<'a> {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new().declaration(Some(true)).open_numbering();
        b = b.add_child(&create_default_numbering());
        for n in &self.numberings {
            b = b.add_child(n);
        }
        b.close().build()
    }
}

fn create_default_numbering() -> Numbering<'static> {
    Numbering::new(0)
        .add_level(
            Level::new(
                0,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%1."),
                LevelJc::new("left"),
            )
            .indent(420, Some(SpecialIndentType::Hanging(420))),
        )
        .add_level(
            Level::new(
                1,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("(%2)"),
                LevelJc::new("left"),
            )
            .indent(840, Some(SpecialIndentType::Hanging(420))),
        )
        .add_level(
            Level::new(
                2,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%3"),
                LevelJc::new("left"),
            )
            .indent(1260, Some(SpecialIndentType::Hanging(420))),
        )
        .add_level(
            Level::new(
                3,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%4."),
                LevelJc::new("left"),
            )
            .indent(1680, Some(SpecialIndentType::Hanging(420))),
        )
        .add_level(
            Level::new(
                4,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("(%5)"),
                LevelJc::new("left"),
            )
            .indent(2100, Some(SpecialIndentType::Hanging(420))),
        )
        .add_level(
            Level::new(
                5,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%6"),
                LevelJc::new("left"),
            )
            .indent(2520, Some(SpecialIndentType::Hanging(420))),
        )
        .add_level(
            Level::new(
                6,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%7."),
                LevelJc::new("left"),
            )
            .indent(2940, Some(SpecialIndentType::Hanging(420))),
        )
        .add_level(
            Level::new(
                7,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("(%8)"),
                LevelJc::new("left"),
            )
            .indent(3360, Some(SpecialIndentType::Hanging(420))),
        )
        .add_level(
            Level::new(
                8,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%9"),
                LevelJc::new("left"),
            )
            .indent(3780, Some(SpecialIndentType::Hanging(420))),
        )
}
