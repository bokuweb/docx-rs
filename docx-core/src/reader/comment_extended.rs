use std::io::Read;

use xml::attribute::OwnedAttribute;
use xml::reader::EventReader;

use super::*;

impl ElementReader for CommentExtended {
    fn read<R: Read>(
        _r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let para_id = read(attrs, "paraId").expect("should comment id exists.");
        let mut comment_extended = CommentExtended::new(para_id);
        if let Some(done) = read(attrs, "done") {
            if !is_false(&done) {
                comment_extended = comment_extended.done();
            }
        };
        if let Some(parent_id) = read(attrs, "paraIdParent") {
            comment_extended = comment_extended.parent_paragraph_id(parent_id);
        }
        Ok(comment_extended)
    }
}
