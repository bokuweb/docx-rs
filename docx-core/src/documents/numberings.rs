use super::*;
use crate::documents::BuildXML;
use crate::types::*;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Numberings {
    abstract_nums: Vec<AbstractNumbering>,
    numberings: Vec<Numbering>,
}

impl Numberings {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_abstract_numbering(mut self, n: AbstractNumbering) -> Self {
        self.abstract_nums.push(n);
        self
    }

    pub fn add_numbering(mut self, n: Numbering) -> Self {
        self.numberings.push(n);
        self
    }
}

impl Default for Numberings {
    fn default() -> Self {
        Self {
            abstract_nums: vec![],
            numberings: vec![],
        }
    }
}

impl BuildXML for Numberings {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new().declaration(Some(true)).open_numbering();
        b = b.add_child(&create_default_numbering());
        for n in &self.abstract_nums {
            b = b.add_child(n);
        }
        b = b.add_child(&Numbering::new(1, 1));
        for n in &self.numberings {
            b = b.add_child(n);
        }
        b.close().build()
    }
}

fn create_default_numbering() -> AbstractNumbering {
    AbstractNumbering::new(1)
        .add_level(
            Level::new(
                0,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%1."),
                LevelJc::new("left"),
            )
            .indent(Some(420), Some(SpecialIndentType::Hanging(420)), None, None),
        )
        .add_level(
            Level::new(
                1,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("(%2)"),
                LevelJc::new("left"),
            )
            .indent(Some(840), Some(SpecialIndentType::Hanging(420)), None, None),
        )
        .add_level(
            Level::new(
                2,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%3"),
                LevelJc::new("left"),
            )
            .indent(
                Some(1260),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                3,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%4."),
                LevelJc::new("left"),
            )
            .indent(
                Some(1680),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                4,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("(%5)"),
                LevelJc::new("left"),
            )
            .indent(
                Some(2100),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                5,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%6"),
                LevelJc::new("left"),
            )
            .indent(
                Some(2520),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                6,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%7."),
                LevelJc::new("left"),
            )
            .indent(
                Some(2940),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                7,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("(%8)"),
                LevelJc::new("left"),
            )
            .indent(
                Some(3360),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                8,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%9"),
                LevelJc::new("left"),
            )
            .indent(
                Some(3780),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
}
