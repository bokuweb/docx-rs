use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::EventReader;

use super::*;

impl ElementReader for PageNumType {
    fn read<R: Read>(
        _r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut p = PageNumType::new();
        for a in attrs {
            let local_name = &a.name.local_name;
            if local_name == "start" {
                if let Ok(s) = u32::from_str(&a.value) {
                    p = p.start(s);
                }
            } else if local_name == "chapStyle" {
                p = p.chap_style(a.value.clone());
            }
        }
        Ok(p)
    }
}
