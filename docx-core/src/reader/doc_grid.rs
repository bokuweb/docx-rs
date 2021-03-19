use std::io::Read;
use std::str::FromStr;

use crate::types::*;
use xml::attribute::OwnedAttribute;
use xml::reader::EventReader;

use super::*;

impl ElementReader for DocGrid {
    fn read<R: Read>(
        _r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut doc_grid = DocGrid::with_empty();
        for a in attrs {
            let local_name = &a.name.local_name;
            if local_name == "type" {
                let t = DocGridType::from_str(&a.value)?;
                doc_grid = doc_grid.grid_type(t);
            } else if local_name == "linePitch" {
                let line_pitch = f32::from_str(&a.value)?;
                doc_grid = doc_grid.line_pitch(line_pitch as usize);
            } else if local_name == "charSpace" {
                let char_space = f32::from_str(&a.value)?;
                doc_grid = doc_grid.char_space(char_space as usize);
            }
        }
        Ok(doc_grid)
    }
}
