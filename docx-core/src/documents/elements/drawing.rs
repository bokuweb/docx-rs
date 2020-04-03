use super::*;
use serde::{Deserialize, Serialize};

use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Drawing {
    pub child: Option<DrawingChild>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum DrawingChild {
    Anchor(Anchor),
}

impl Drawing {
    pub fn new() -> Drawing {
        Default::default()
    }

    pub fn anchor(mut self, a: Anchor) -> Drawing {
        self.child = Some(DrawingChild::Anchor(a));
        self
    }
}

impl Default for Drawing {
    fn default() -> Self {
        Drawing { child: None }
    }
}

impl BuildXML for Drawing {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b.open_drawing();
        match &self.child {
            Some(DrawingChild::Anchor(a)) => {
                b = b.add_child(a);
            }
            None => {}
        }
        b.close().build()
    }
}
