mod a_graphic;
mod a_graphic_data;
mod attributes;
mod bookmark_end;
mod bookmark_start;
mod comment;
mod comment_extended;
mod comments;
mod comments_extended;
mod delete;
mod doc_defaults;
mod document;
mod document_rels;
mod drawing;
mod errors;
mod from_xml;
mod ignore;
mod table_cell_property;
mod insert;
mod level;
mod level_override;
mod mc_fallback;
mod numbering_property;
mod numberings;
mod paragraph;
mod read_zip;
mod rels;
mod doc_grid;
mod run;
mod run_property;
mod section_property;
mod settings;
mod shading;
mod style;
mod styles;
mod table;
mod table_borders;
mod table_cell;
mod table_cell_borders;
mod table_property;
mod table_row;
mod text_box_content;
mod wp_anchor;
mod wps_shape;
mod wps_text_box;
mod xml_element;

use std::io::Cursor;

use crate::documents::*;

pub use attributes::*;
pub use document_rels::*;
pub use errors::ReaderError;
pub use from_xml::*;
pub use mc_fallback::*;
pub use read_zip::*;
pub use xml_element::*;

const DOC_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const STYLE_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
const NUMBERING_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering";
const SETTINGS_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings";
const COMMENTS_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments";
const COMMENTS_EXTENDED_TYPE: &str =
    "http://schemas.microsoft.com/office/2011/relationships/commentsExtended";

pub fn read_docx(buf: &[u8]) -> Result<Docx, ReaderError> {
    let cur = Cursor::new(buf);
    let mut archive = zip::ZipArchive::new(cur)?;
    // First, the content type for relationship parts and the Main Document part
    // (the only required part) must be defined (physically located at /[Content_Types].xml in the package)
    let _content_types = {
        let data = read_zip(&mut archive, "[Content_Types].xml")?;
        ContentTypes::from_xml(&data[..])?
    };

    // Next, the single required relationship (the package-level relationship to the Main Document part)
    //  must be defined (physically located at /_rels/.rels in the package)
    let rels = {
        let data = read_zip(&mut archive, "_rels/.rels")?;
        Rels::from_xml(&data[..])?
    };
    // Finally, the minimum content for the Main Document part must be defined
    // (physically located at /document.xml in the package):
    let main_rel = rels
        .find_target(DOC_RELATIONSHIP_TYPE)
        .ok_or(ReaderError::DocumentNotFoundError);

    let document_path = if let Ok(rel) = main_rel {
        rel.2.clone()
    } else {
        "word/document.xml".to_owned()
    };

    let rels = read_document_rels(&mut archive, &document_path)?;

    // Read commentsExtended
    let comments_extended_path = rels.find_target_path(COMMENTS_EXTENDED_TYPE);
    let comments_extended = if let Some(comments_extended_path) = comments_extended_path {
        let data = read_zip(
            &mut archive,
            comments_extended_path
                .to_str()
                .expect("should have comments extended."),
        );
        if let Ok(data) = data {
            CommentsExtended::from_xml(&data[..])?
        } else {
            CommentsExtended::default()
        }
    } else {
        CommentsExtended::default()
    };

    // Read comments
    let comments_path = rels.find_target_path(COMMENTS_TYPE);
    let comments = if let Some(comments_path) = comments_path {
        let data = read_zip(
            &mut archive,
            comments_path.to_str().expect("should have comments."),
        );
        if let Ok(data) = data {
            let mut comments = Comments::from_xml(&data[..])?.into_inner();
            for i in 0..comments.len() {
                let c = &comments[i];
                let extended = comments_extended.children.iter().find(|ex| {
                    for child in &c.children {
                        if let CommentChild::Paragraph(p) = child {
                            if ex.paragraph_id == p.id {
                                return true;
                            }
                        }
                    }
                    false
                });
                if let Some(CommentExtended {
                    parent_paragraph_id: Some(parent_paragraph_id),
                    ..
                }) = extended
                {
                    if let Some(parent_comment) = comments.iter().find(|c| {
                        for child in &c.children {
                            if let CommentChild::Paragraph(p) = child {
                                if &p.id == parent_paragraph_id {
                                    return true;
                                }
                            }
                        }
                        false
                    }) {
                        comments[i].parent_comment_id = Some(parent_comment.id);
                    }
                }
            }
            Comments { comments }
        } else {
            Comments::default()
        }
    } else {
        Comments::default()
    };

    let document = {
        let data = read_zip(&mut archive, &document_path)?;
        Document::from_xml(&data[..])?
    };
    let mut docx = Docx::new().document(document);

    // store comments to paragraphs.
    if !comments.inner().is_empty() {
        docx.store_comments(comments.inner());
        docx = docx.comments(comments);
        docx = docx.comments_extended(comments_extended);
    }

    // Read document relationships
    // Read styles
    let style_path = rels.find_target_path(STYLE_RELATIONSHIP_TYPE);
    if let Some(style_path) = style_path {
        let data = read_zip(
            &mut archive,
            style_path.to_str().expect("should have styles"),
        )?;
        let styles = Styles::from_xml(&data[..])?;
        docx = docx.styles(styles);
    }

    // Read numberings
    let num_path = rels.find_target_path(NUMBERING_RELATIONSHIP_TYPE);
    if let Some(num_path) = num_path {
        let data = read_zip(
            &mut archive,
            num_path.to_str().expect("should have numberings"),
        )?;
        let nums = Numberings::from_xml(&data[..])?;
        docx = docx.numberings(nums);
    }

    // Read settings
    let settings_path = rels.find_target_path(SETTINGS_TYPE);
    if let Some(settings_path) = settings_path {
        let data = read_zip(
            &mut archive,
            settings_path.to_str().expect("should have settings"),
        )?;
        let settings = Settings::from_xml(&data[..])?;
        docx = docx.settings(settings);
    }

    Ok(docx)
}
