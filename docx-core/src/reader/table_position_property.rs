use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for TablePositionProperty {
    fn read<R: Read>(
        _r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = TablePositionProperty::new();
        for a in attrs {
            let local_name = &a.name.local_name;
            match local_name.as_str() {
                "leftFromText" => {
                    if let Ok(v) = i32::from_str(&a.value) {
                        property = property.left_from_text(v);
                    }
                }
                "rightFromText" => {
                    if let Ok(v) = i32::from_str(&a.value) {
                        property = property.right_from_text(v);
                    }
                }
                "vertAnchor" => {
                    property = property.vertical_anchor(a.value.clone());
                }
                "horzAnchor" => {
                    property = property.horizontal_anchor(a.value.clone());
                }
                "tblpXSpec" => {
                    property = property.position_x_alignment(a.value.clone());
                }
                "tblpYSpec" => {
                    property = property.position_y_alignment(a.value.clone());
                }
                "tblpX" => {
                    if let Ok(v) = i32::from_str(&a.value) {
                        property = property.position_x(v);
                    }
                }
                "tblpY" => {
                    if let Ok(v) = i32::from_str(&a.value) {
                        property = property.position_y(v);
                    }
                }
                _ => {}
            }
        }
        Ok(property)
    }
}
