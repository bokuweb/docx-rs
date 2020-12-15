use super::Comment;
use crate::documents::BuildXML;
use crate::xml_builder::*;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Comments {
    pub(crate) comments: Vec<Comment>,
}

impl Comments {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn inner(&self) -> &[Comment] {
        &self.comments
    }

    pub fn into_inner(self) -> Vec<Comment> {
        self.comments
    }

    pub(crate) fn add_comments(&mut self, comments: Vec<Comment>) {
        self.comments = comments;
    }
}

impl Default for Comments {
    fn default() -> Self {
        Self { comments: vec![] }
    }
}

impl BuildXML for Comments {
    fn build(&self) -> Vec<u8> {
        let mut b = XMLBuilder::new().declaration(Some(true)).open_comments();
        for c in &self.comments {
            b = b.add_child(c)
        }
        b.close().build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_comments() {
        let b = Comments::new().build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:comments xmlns:o="urn:schemas-microsoft-com:office:office" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:w10="urn:schemas-microsoft-com:office:word" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup" xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006" xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing" xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml" mc:Ignorable="w14 wp14" />"#
        );
    }
}
