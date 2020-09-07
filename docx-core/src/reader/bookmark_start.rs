use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::EventReader;

use super::*;

impl ElementReader for BookmarkStart {
    fn read<R: Read>(
        _r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut id: Option<usize> = None;
        let mut name: Option<String> = None;

        for a in attrs {
            let local_name = &a.name.local_name;
            if local_name == "id" {
                id = Some(usize::from_str(&a.value)?);
            } else if local_name == "name" {
                name = Some(a.value.clone());
            }
        }
        if id.is_none() || name.is_none() {
            return Err(ReaderError::XMLReadError);
        }
        Ok(BookmarkStart::new(id.unwrap(), name.unwrap()))
    }
}
