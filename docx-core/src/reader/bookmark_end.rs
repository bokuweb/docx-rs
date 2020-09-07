use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::EventReader;

use super::*;

impl ElementReader for BookmarkEnd {
    fn read<R: Read>(
        _r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut id: Option<usize> = None;
        for a in attrs {
            let local_name = &a.name.local_name;
            if local_name == "id" {
                id = Some(usize::from_str(&a.value)?);
            }
        }
        if let Some(id) = id {
            Ok(BookmarkEnd::new(id))
        } else {
            Err(ReaderError::XMLReadError)
        }
    }
}
