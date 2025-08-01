mod a_graphic;
mod a_graphic_data;
mod attributes;
mod bookmark_end;
mod bookmark_start;
mod cell_margins;
mod comment;
mod comment_extended;
mod comments;
mod comments_extended;
mod custom_properties;
mod delete;
mod div;
mod doc_defaults;
mod doc_grid;
mod document;
mod document_rels;
mod drawing;
mod errors;
mod font_group;
mod font_scheme;
mod footer;
mod frame_property;
mod from_xml;
mod header;
mod header_or_footer_rels;
mod hyperlink;
mod ignore;
mod insert;
mod level;
mod level_override;
mod mc_fallback;
mod numbering_property;
mod numberings;
mod page_num_type;
mod paragraph;
mod paragraph_property;
mod paragraph_property_change;
mod pic;
mod positional_tab;
mod read_zip;
mod rels;
mod run;
mod run_property;
mod section_property;
mod settings;
mod shading;
mod shape;
mod structured_data_tag;
mod style;
mod styles;
mod tab;
mod table;
mod table_borders;
mod table_cell;
mod table_cell_borders;
mod table_cell_margins;
mod table_cell_property;
mod table_position_property;
mod table_property;
mod table_row;
mod tabs;
mod text_box_content;
mod theme;
mod web_settings;
mod wp_anchor;
mod wps_shape;
mod wps_text_box;
mod xml_element;

use std::{collections::HashMap, io::Cursor, path::PathBuf};

use crate::documents::*;

pub use attributes::*;
pub use document_rels::*;
pub use errors::ReaderError;
pub use from_xml::*;
pub use read_zip::*;
pub use xml_element::*;
use zip::ZipArchive;

use self::header_or_footer_rels::{read_header_or_footer_rels, ReadHeaderOrFooterRels};

// 2006
const DOC_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const CUSTOM_PROPERTIES_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties";
const STYLE_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles";
const NUMBERING_RELATIONSHIP_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering";
const SETTINGS_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings";
const COMMENTS_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments";
const WEB_SETTINGS_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/webSettings";
const HEADER_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/header";
const FOOTER_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer";
const THEME_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme";
const IMAGE_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image";
const HYPERLINK_TYPE: &str =
    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink";
// 2011
const COMMENTS_EXTENDED_TYPE: &str =
    "http://schemas.microsoft.com/office/2011/relationships/commentsExtended";

fn read_headers(
    rels: &ReadDocumentRels,
    archive: &mut ZipArchive<Cursor<&[u8]>>,
) -> HashMap<RId, (Header, ReadHeaderOrFooterRels)> {
    let header_paths = rels.find_target_path(HEADER_TYPE);
    let headers: HashMap<RId, (Header, ReadHeaderOrFooterRels)> = header_paths
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(rid, path, ..)| {
            let data = read_zip(archive, path.to_str().expect("should have header path."));
            if let Ok(d) = data {
                if let Ok(h) = Header::from_xml(&d[..]) {
                    let rels = read_header_or_footer_rels(archive, path).unwrap_or_default();
                    return Some((rid, (h, rels)));
                }
            }
            None
        })
        .collect();
    headers
}

fn read_footers(
    rels: &ReadDocumentRels,
    archive: &mut ZipArchive<Cursor<&[u8]>>,
) -> HashMap<RId, (Footer, ReadHeaderOrFooterRels)> {
    let footer_paths = rels.find_target_path(FOOTER_TYPE);
    let footers: HashMap<RId, (Footer, ReadHeaderOrFooterRels)> = footer_paths
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(rid, path, ..)| {
            let data = read_zip(archive, path.to_str().expect("should have footer path."));
            if let Ok(d) = data {
                if let Ok(h) = Footer::from_xml(&d[..]) {
                    let rels = read_header_or_footer_rels(archive, path).unwrap_or_default();
                    return Some((rid, (h, rels)));
                }
            }
            None
        })
        .collect();
    footers
}

fn read_themes(rels: &ReadDocumentRels, archive: &mut ZipArchive<Cursor<&[u8]>>) -> Vec<Theme> {
    let theme_paths = rels.find_target_path(THEME_TYPE);
    theme_paths
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(_rid, path, ..)| {
            let data = read_zip(archive, path.to_str().expect("should have footer path."));
            if let Ok(d) = data {
                if let Ok(h) = Theme::from_xml(&d[..]) {
                    return Some(h);
                }
            }
            None
        })
        .collect()
}

pub fn read_docx(buf: &[u8]) -> Result<Docx, ReaderError> {
    let mut docx = Docx::new();
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

    if let Some(custom_props) = rels.find_target(CUSTOM_PROPERTIES_TYPE) {
        let data = read_zip(&mut archive, &custom_props.2);
        if let Ok(data) = data {
            if let Ok(custom) = CustomProps::from_xml(&data[..]) {
                docx.doc_props.custom = custom;
            }
        }
    }

    let rels = read_document_rels(&mut archive, &document_path)?;

    let headers = read_headers(&rels, &mut archive);
    let footers = read_footers(&rels, &mut archive);

    docx.themes = read_themes(&rels, &mut archive);

    // Read commentsExtended
    let comments_extended_path = rels.find_target_path(COMMENTS_EXTENDED_TYPE);
    let comments_extended = if let Some(comments_extended_path) = comments_extended_path {
        if let Some((_, comments_extended_path, ..)) = comments_extended_path.first() {
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
        }
    } else {
        CommentsExtended::default()
    };

    // Read comments
    let comments_path = rels.find_target_path(COMMENTS_TYPE);
    let comments = if let Some(paths) = comments_path {
        if let Some((_, comments_path, ..)) = paths.first() {
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
        }
    } else {
        Comments::default()
    };

    let document = {
        let data = read_zip(&mut archive, &document_path)?;
        Document::from_xml(&data[..])?
    };
    docx = docx.document(document);

    // assign headers
    if let Some(h) = docx.document.section_property.header_reference.clone() {
        if let Some((header, rels)) = headers.get(&h.id) {
            docx.document = docx.document.header(header.clone(), &h.id);
            let count = docx.document_rels.header_count + 1;
            docx.document_rels.header_count = count;
            docx.content_type = docx.content_type.add_header();
            // Read media
            let media = rels.find_target_path(IMAGE_TYPE);
            docx = add_images(docx, media, &mut archive);
        }
    }
    if let Some(ref h) = docx
        .document
        .section_property
        .first_header_reference
        .clone()
    {
        if let Some((header, rels)) = headers.get(&h.id) {
            docx.document = docx
                .document
                .first_header_without_title_pg(header.clone(), &h.id);
            let count = docx.document_rels.header_count + 1;
            docx.document_rels.header_count = count;
            docx.content_type = docx.content_type.add_header();
            // Read media
            let media = rels.find_target_path(IMAGE_TYPE);
            docx = add_images(docx, media, &mut archive);
        }
    }
    if let Some(ref h) = docx.document.section_property.even_header_reference.clone() {
        if let Some((header, rels)) = headers.get(&h.id) {
            docx.document = docx.document.even_header(header.clone(), &h.id);
            let count = docx.document_rels.header_count + 1;
            docx.document_rels.header_count = count;
            docx.content_type = docx.content_type.add_header();

            // Read media
            let media = rels.find_target_path(IMAGE_TYPE);
            docx = add_images(docx, media, &mut archive);
        }
    }

    // assign footers
    if let Some(f) = docx.document.section_property.footer_reference.clone() {
        if let Some((footer, rels)) = footers.get(&f.id) {
            docx.document = docx.document.footer(footer.clone(), &f.id);
            let count = docx.document_rels.footer_count + 1;
            docx.document_rels.footer_count = count;
            docx.content_type = docx.content_type.add_footer();

            // Read media
            let media = rels.find_target_path(IMAGE_TYPE);
            docx = add_images(docx, media, &mut archive);
        }
    }

    if let Some(ref f) = docx
        .document
        .section_property
        .first_footer_reference
        .clone()
    {
        if let Some((footer, rels)) = footers.get(&f.id) {
            docx.document = docx
                .document
                .first_footer_without_title_pg(footer.clone(), &f.id);
            let count = docx.document_rels.footer_count + 1;
            docx.document_rels.footer_count = count;
            docx.content_type = docx.content_type.add_footer();

            // Read media
            let media = rels.find_target_path(IMAGE_TYPE);
            docx = add_images(docx, media, &mut archive);
        }
    }
    if let Some(ref f) = docx.document.section_property.even_footer_reference.clone() {
        if let Some((footer, rels)) = footers.get(&f.id) {
            docx.document = docx.document.even_footer(footer.clone(), &f.id);
            let count = docx.document_rels.footer_count + 1;
            docx.document_rels.footer_count = count;
            docx.content_type = docx.content_type.add_footer();

            // Read media
            let media = rels.find_target_path(IMAGE_TYPE);
            docx = add_images(docx, media, &mut archive);
        }
    }

    // store comments to paragraphs.
    if !comments.inner().is_empty() {
        docx.store_comments(comments.inner());
        docx = docx.comments(comments);
        docx = docx.comments_extended(comments_extended);
    }

    // Read document relationships
    // Read styles
    let style_path = rels.find_target_path(STYLE_RELATIONSHIP_TYPE);
    if let Some(paths) = style_path {
        if let Some((_, style_path, ..)) = paths.first() {
            let data = read_zip(
                &mut archive,
                style_path.to_str().expect("should have styles"),
            )?;
            let styles = Styles::from_xml(&data[..])?;
            docx = docx.styles(styles);
        }
    }

    // Read numberings
    let num_path = rels.find_target_path(NUMBERING_RELATIONSHIP_TYPE);
    if let Some(paths) = num_path {
        if let Some((_, num_path, ..)) = paths.first() {
            let data = read_zip(
                &mut archive,
                num_path.to_str().expect("should have numberings"),
            )?;
            let nums = Numberings::from_xml(&data[..])?;
            docx = docx.numberings(nums);
        }
    }

    // Read settings
    let settings_path = rels.find_target_path(SETTINGS_TYPE);
    if let Some(paths) = settings_path {
        if let Some((_, settings_path, ..)) = paths.first() {
            let data = read_zip(
                &mut archive,
                settings_path.to_str().expect("should have settings"),
            )?;
            let settings = Settings::from_xml(&data[..])?;
            docx = docx.settings(settings);
        }
    }

    // Read web settings
    let web_settings_path = rels.find_target_path(WEB_SETTINGS_TYPE);
    if let Some(paths) = web_settings_path {
        if let Some((_, web_settings_path, ..)) = paths.first() {
            let data = read_zip(
                &mut archive,
                web_settings_path
                    .to_str()
                    .expect("should have web settings"),
            )?;
            let web_settings = WebSettings::from_xml(&data[..])?;
            docx = docx.web_settings(web_settings);
        }
    }
    // Read media
    let media = rels.find_target_path(IMAGE_TYPE);
    docx = add_images(docx, media, &mut archive);

    // Read hyperlinks
    let links = rels.find_target_path(HYPERLINK_TYPE);
    if let Some(paths) = links {
        for (id, target, mode) in paths {
            if let Some(mode) = mode {
                docx =
                    docx.add_hyperlink(id, target.to_str().expect("should convert to str"), mode);
            }
        }
    }

    Ok(docx)
}

fn add_images(
    mut docx: Docx,
    media: Option<Vec<(RId, PathBuf, Option<String>)>>,
    archive: &mut ZipArchive<Cursor<&[u8]>>,
) -> Docx {
    // Read media
    if let Some(paths) = media {
        for (id, media, ..) in paths {
            if let Ok(data) = read_zip(archive, media.to_str().expect("should have media")) {
                docx = docx.add_image(id, media.to_str().unwrap().to_string(), data);
            }
        }
    }
    docx
}

/// A struct to hold information about a part in an XML package
#[derive(Debug, Clone)]
pub struct XmlPackagePart {
    pub name: String,
    pub _content_type: String,
    pub data: String,
}

/// Decode HTML entities
fn decode_html_entities(text: &str) -> String {
    text.replace("&quot;", "\"")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&apos;", "'")
}

/// Extract parts from a Microsoft Word XML package using simple string parsing
pub fn extract_xml_package_parts(xml_content: &str) -> Result<Vec<XmlPackagePart>, ReaderError> {
    let mut parts = Vec::new();
    
    // Find all pkg:part elements
    let mut start_idx = 0;
    while let Some(part_start) = xml_content[start_idx..].find("<pkg:part") {
        let part_start = start_idx + part_start;
        
        // Find the end of this part
        if let Some(part_end) = xml_content[part_start..].find("</pkg:part>") {
            let part_end = part_start + part_end + 11; // 11 = "</pkg:part>".len()
            let part_xml = &xml_content[part_start..part_end];
            
            // Extract name and contentType attributes (handle HTML entities)
            let name = if let Some(name_start) = part_xml.find("pkg:name=\"\"") {
                let name_start = name_start + 12; // 12 = "pkg:name=\"\"".len()
                if let Some(name_end) = part_xml[name_start..].find("\"\"") {
                    decode_html_entities(&part_xml[name_start..name_start + name_end])
                } else {
                    continue;
                }
            } else {
                continue;
            };
            
            let content_type = if let Some(type_start) = part_xml.find("pkg:contentType=\"\"") {
                let type_start = type_start + 19; // 19 = "pkg:contentType=\"\"".len()
                if let Some(type_end) = part_xml[type_start..].find("\"\"") {
                    decode_html_entities(&part_xml[type_start..type_start + type_end])
                } else {
                    continue;
                }
            } else {
                continue;
            };
            
            // Extract xmlData content
            let data = if let Some(data_start) = part_xml.find("<pkg:xmlData>") {
                let data_start = data_start + 13; // 13 = "<pkg:xmlData>".len()
                if let Some(data_end) = part_xml[data_start..].find("</pkg:xmlData>") {
                    decode_html_entities(&part_xml[data_start..data_start + data_end])
                } else {
                    continue;
                }
            } else {
                continue;
            };
            
            parts.push(XmlPackagePart {
                name,
                _content_type: content_type,
                data,
            });
            
            start_idx = part_end;
        } else {
            break;
        }
    }
    
    Ok(parts)
}

/// Read a Docx from Microsoft Word XML format (single XML file with all parts)
pub fn read_docx_from_xml(xml_content: &str) -> Result<Docx, ReaderError> {
    let mut docx = Docx::new();
    
    // Extract all parts from the XML package
    let parts = extract_xml_package_parts(xml_content)?;
    
    // Create a HashMap for easy lookup of parts by name
    let mut part_map: HashMap<String, String> = HashMap::new();
    for part in parts {
        part_map.insert(part.name, part.data);
    }
    
    // Read main relationships
    let rels = if let Some(rels_data) = part_map.get("_rels/.rels") {
        Rels::from_xml(rels_data.as_bytes())?
    } else {
        return Err(ReaderError::DocumentNotFoundError);
    };
    
    // Find the main document path
    let main_rel = rels
        .find_target(DOC_RELATIONSHIP_TYPE)
        .ok_or(ReaderError::DocumentNotFoundError);
    
    let document_path = if let Ok(rel) = main_rel {
        rel.2.clone()
    } else {
        "word/document.xml".to_owned()
    };
    
    // Read custom properties if available
    if let Some(custom_props) = rels.find_target(CUSTOM_PROPERTIES_TYPE) {
        if let Some(data) = part_map.get(&custom_props.2) {
            if let Ok(custom) = CustomProps::from_xml(data.as_bytes()) {
                docx.doc_props.custom = custom;
            }
        }
    }
    
    // Read document relationships
    let document_rels_path = document_path.replace("document.xml", "_rels/document.xml.rels");
    let document_rels = if let Some(rels_data) = part_map.get(&document_rels_path) {
        Rels::from_xml(rels_data.as_bytes())?
    } else {
        Rels::default()
    };
    
    // Read themes (basic implementation)
    if let Some(theme_rel) = document_rels.find_target(THEME_TYPE) {
        let theme_path_str = format!("{}/{}", 
            document_path.replace("document.xml", ""), 
            theme_rel.2
        );
        if let Some(theme_data) = part_map.get(&theme_path_str) {
            if let Ok(theme) = Theme::from_xml(theme_data.as_bytes()) {
                docx.themes.push(theme);
            }
        }
    }
    
    // Read the main document
    let document = if let Some(doc_data) = part_map.get(&document_path) {
        Document::from_xml(doc_data.as_bytes())?
    } else {
        return Err(ReaderError::DocumentNotFoundError);
    };
    
    docx = docx.document(document);
    
    // Read styles if available (basic implementation)
    if let Some(styles_rel) = document_rels.find_target(STYLE_RELATIONSHIP_TYPE) {
        let styles_path_str = format!("{}/{}", 
            document_path.replace("document.xml", ""), 
            styles_rel.2
        );
        if let Some(styles_data) = part_map.get(&styles_path_str) {
            if let Ok(styles) = Styles::from_xml(styles_data.as_bytes()) {
                docx.styles = styles;
            }
        }
    }
    
    // Read numbering if available (basic implementation)
    if let Some(numbering_rel) = document_rels.find_target(NUMBERING_RELATIONSHIP_TYPE) {
        let numbering_path_str = format!("{}/{}", 
            document_path.replace("document.xml", ""), 
            numbering_rel.2
        );
        if let Some(numbering_data) = part_map.get(&numbering_path_str) {
            if let Ok(numberings) = Numberings::from_xml(numbering_data.as_bytes()) {
                docx.numberings = numberings;
            }
        }
    }
    
    // Read settings if available (basic implementation)
    if let Some(settings_rel) = document_rels.find_target(SETTINGS_TYPE) {
        let settings_path_str = format!("{}/{}", 
            document_path.replace("document.xml", ""), 
            settings_rel.2
        );
        if let Some(settings_data) = part_map.get(&settings_path_str) {
            if let Ok(settings) = Settings::from_xml(settings_data.as_bytes()) {
                docx.settings = settings;
            }
        }
    }
    
    // Read web settings if available (basic implementation)
    if let Some(web_settings_rel) = document_rels.find_target(WEB_SETTINGS_TYPE) {
        let web_settings_path_str = format!("{}/{}", 
            document_path.replace("document.xml", ""), 
            web_settings_rel.2
        );
        if let Some(web_settings_data) = part_map.get(&web_settings_path_str) {
            if let Ok(web_settings) = WebSettings::from_xml(web_settings_data.as_bytes()) {
                docx.web_settings = web_settings;
            }
        }
    }
    
    Ok(docx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_docx_from_xml_basic() {
        // A minimal XML package with just the basic structure
        let xml_content = r#"<?xml version="1.0" standalone="yes"?>
<pkg:package xmlns:pkg="http://schemas.microsoft.com/office/2006/xmlPackage">
    <pkg:part pkg:name="/_rels/.rels" pkg:contentType="application/vnd.openxmlformats-package.relationships+xml">
        <pkg:xmlData>
            <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
                <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
            </Relationships>
        </pkg:xmlData>
    </pkg:part>
    <pkg:part pkg:name="/word/document.xml" pkg:contentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml">
        <pkg:xmlData>
            <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
                <w:body>
                    <w:p>
                        <w:r>
                            <w:t>Hello, World!</w:t>
                        </w:r>
                    </w:p>
                </w:body>
            </w:document>
        </pkg:xmlData>
    </pkg:part>
</pkg:package>"#;

        let result = read_docx_from_xml(xml_content);
        assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());
        
        let docx = result.unwrap();
        assert!(!docx.document.children.is_empty(), "Document should contain some content");
    }

    #[test]
    fn test_extract_xml_package_parts() {
        let xml_content = r#"<pkg:package xmlns:pkg="http://schemas.microsoft.com/office/2006/xmlPackage">
    <pkg:part pkg:name="/_rels/.rels" pkg:contentType="application/vnd.openxmlformats-package.relationships+xml">
        <pkg:xmlData>
            <test>content</test>
        </pkg:xmlData>
    </pkg:part>
</pkg:package>"#;

        let result = extract_xml_package_parts(xml_content);
        assert!(result.is_ok());
        
        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].name, "/_rels/.rels");
        assert!(parts[0].data.contains("<test>content</test>"));
    }
}
