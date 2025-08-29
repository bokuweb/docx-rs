use super::header_or_footer_rels::ReadHeaderOrFooterRels;
use super::namespace::*;
use super::*;

fn read_headers_from_xml(
    rels: &Rels,
    part_map: &HashMap<String, String>,
    document_path: &str,
) -> HashMap<RId, (Header, ReadHeaderOrFooterRels)> {
    let mut headers = HashMap::new();

    // Find all header relationships by looking for header types in rels
    for (rel_type, id, target) in &rels.rels {
        if rel_type == HEADER_TYPE {
            let header_path = format!("{}/{}", document_path.replace("document.xml", ""), target);

            if let Some(header_data) = part_map.get(&header_path) {
                if let Ok(header) = Header::from_xml(header_data.as_bytes()) {
                    // For simplicity, use default ReadHeaderOrFooterRels
                    // In a full implementation, we would read the header's _rels file
                    let header_rels = ReadHeaderOrFooterRels::default();
                    headers.insert(id.clone(), (header, header_rels));
                }
            }
        }
    }

    headers
}

fn read_footers_from_xml(
    rels: &Rels,
    part_map: &HashMap<String, String>,
    document_path: &str,
) -> HashMap<RId, (Footer, ReadHeaderOrFooterRels)> {
    let mut footers = HashMap::new();

    // Find all footer relationships by looking for footer types in rels
    for (rel_type, id, target) in &rels.rels {
        if rel_type == FOOTER_TYPE {
            let footer_path = format!("{}/{}", document_path.replace("document.xml", ""), target);

            if let Some(footer_data) = part_map.get(&footer_path) {
                if let Ok(footer) = Footer::from_xml(footer_data.as_bytes()) {
                    // For simplicity, use default ReadHeaderOrFooterRels
                    // In a full implementation, we would read the footer's _rels file
                    let footer_rels = ReadHeaderOrFooterRels::default();
                    footers.insert(id.clone(), (footer, footer_rels));
                }
            }
        }
    }

    footers
}

fn read_comments_from_xml(
    rels: &Rels,
    part_map: &HashMap<String, String>,
    document_path: &str,
) -> (Comments, CommentsExtended) {
    // Simplified implementation for XML packages - try to read comments if available
    let comments_extended = if let Some((_, _, target)) = rels.find_target(COMMENTS_EXTENDED_TYPE) {
        let ext_path = format!("{}/{}", document_path.replace("document.xml", ""), target);
        if let Some(comments_ext_data) = part_map.get(&ext_path) {
            CommentsExtended::from_xml(comments_ext_data.as_bytes()).unwrap_or_default()
        } else {
            CommentsExtended::default()
        }
    } else {
        CommentsExtended::default()
    };

    // Read comments
    let comments = if let Some((_, _, target)) = rels.find_target(COMMENTS_TYPE) {
        let comm_path = format!("{}/{}", document_path.replace("document.xml", ""), target);
        if let Some(comments_data) = part_map.get(&comm_path) {
            if let Ok(comments) = Comments::from_xml(comments_data.as_bytes()) {
                // Process extended comments relationships
                let mut comments_inner = comments.into_inner();
                for i in 0..comments_inner.len() {
                    let c = &comments_inner[i];
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
                        if let Some(parent_comment) = comments_inner.iter().find(|c| {
                            for child in &c.children {
                                if let CommentChild::Paragraph(p) = child {
                                    if &p.id == parent_paragraph_id {
                                        return true;
                                    }
                                }
                            }
                            false
                        }) {
                            comments_inner[i].parent_comment_id = Some(parent_comment.id);
                        }
                    }
                }
                Comments {
                    comments: comments_inner,
                }
            } else {
                Comments::default()
            }
        } else {
            Comments::default()
        }
    } else {
        Comments::default()
    };

    (comments, comments_extended)
}

fn add_images_from_xml(
    mut docx: Docx,
    media: Option<Vec<(RId, PathBuf, Option<String>)>>,
    part_map: &HashMap<String, String>,
    document_path: &str,
) -> Docx {
    // Read media from XML package
    if let Some(paths) = media {
        for (id, media_path, ..) in paths {
            let base_path = document_path
                .replace("document.xml", "")
                .trim_end_matches('/')
                .to_string();
            let image_path = format!("{}/{}", base_path, media_path.to_str().unwrap_or(""));

            // Try multiple possible paths for the image
            let paths_to_try = vec![
                image_path.clone(),
                format!("/{}", image_path.trim_start_matches('/')),
                image_path.trim_start_matches('/').to_string(),
            ];

            for path in paths_to_try {
                if let Some(image_data) = part_map.get(&path) {
                    // For XML packages with binaryData, the data is base64 encoded
                    // Remove all whitespace and newlines from base64 data
                    let clean_base64 = image_data
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect::<String>();
                    let bytes = if let Ok(decoded) =
                        base64::engine::general_purpose::STANDARD.decode(&clean_base64)
                    {
                        decoded
                    } else {
                        // If base64 decode fails, try as raw bytes (shouldn't happen for binaryData)
                        image_data.as_bytes().to_vec()
                    };
                    docx =
                        docx.add_image(id.clone(), media_path.to_str().unwrap().to_string(), bytes);
                    break;
                }
            }
        }
    }
    docx
}

fn add_header_footer_images(
    docx: Docx,
    _header_footer_rels: &ReadHeaderOrFooterRels,
    _part_map: &HashMap<String, String>,
    _document_path: &str,
) -> Docx {
    // For XML packages, header/footer image handling is simplified
    // In a full implementation, we would process header/footer relationships for images
    // For now, we'll use the default ReadHeaderOrFooterRels which doesn't have images
    docx
}

/// A struct to hold information about a part in an XML package
#[derive(Debug, Clone)]
pub struct XmlPackagePart {
    pub name: String,
    pub _content_type: String,
    pub data: String,
}

/// Decode HTML entities and fix double-quote issues in Word XML
fn decode_html_entities(text: &str) -> String {
    // The main issue: Word XML uses double quotes incorrectly - fix them first
    let text = text.replace("\"\"", "\"");

    // Then handle standard HTML entities
    text.replace("&quot;", "\"")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&apos;", "'")
        .replace("&#39;", "'")
        .replace("&#34;", "\"")
        .replace("&#38;", "&")
        .replace("&#60;", "<")
        .replace("&#62;", ">")
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

            // Extract name attribute (handle both standard and double-quoted patterns)
            let name = if let Some(name_start) = part_xml.find("pkg:name=\"\"") {
                // Handle double-quoted pattern: pkg:name=""value""
                let name_start = name_start + 12; // 12 = "pkg:name=\"\"".len()
                if let Some(name_end) = part_xml[name_start..].find("\"\"") {
                    decode_html_entities(&part_xml[name_start..name_start + name_end])
                } else {
                    // Skip this part but continue processing
                    start_idx = part_end;
                    continue;
                }
            } else if let Some(name_start) = part_xml.find("pkg:name=\"") {
                // Handle standard pattern: pkg:name="value"
                let name_start = name_start + 10; // 10 = "pkg:name=\"".len()
                if let Some(name_end) = part_xml[name_start..].find("\"") {
                    part_xml[name_start..name_start + name_end].to_string()
                } else {
                    // Skip this part but continue processing
                    start_idx = part_end;
                    continue;
                }
            } else {
                // Skip this part but continue processing
                start_idx = part_end;
                continue;
            };

            // Extract contentType attribute (handle both standard and double-quoted patterns)
            let content_type = if let Some(type_start) = part_xml.find("pkg:contentType=\"\"") {
                // Handle double-quoted pattern: pkg:contentType=""value""
                let type_start = type_start + 19; // 19 = "pkg:contentType=\"\"".len()
                if let Some(type_end) = part_xml[type_start..].find("\"\"") {
                    decode_html_entities(&part_xml[type_start..type_start + type_end])
                } else {
                    // Skip this part but continue processing
                    start_idx = part_end;
                    continue;
                }
            } else if let Some(type_start) = part_xml.find("pkg:contentType=\"") {
                // Handle standard pattern: pkg:contentType="value"
                let type_start = type_start + 17; // 17 = "pkg:contentType=\"".len()
                if let Some(type_end) = part_xml[type_start..].find("\"") {
                    part_xml[type_start..type_start + type_end].to_string()
                } else {
                    // Skip this part but continue processing
                    start_idx = part_end;
                    continue;
                }
            } else {
                // Skip this part but continue processing
                start_idx = part_end;
                continue;
            };

            // Extract xmlData or binaryData content
            let data = if let Some(data_start) = part_xml.find("<pkg:xmlData>") {
                let data_start = data_start + 13; // 13 = "<pkg:xmlData>".len()
                if let Some(data_end) = part_xml[data_start..].find("</pkg:xmlData>") {
                    part_xml[data_start..data_start + data_end].to_string()
                } else {
                    // Skip this part but continue processing
                    start_idx = part_end;
                    continue;
                }
            } else if let Some(data_start) = part_xml.find("<pkg:binaryData>") {
                let data_start = data_start + 16; // 16 = "<pkg:binaryData>".len()
                if let Some(data_end) = part_xml[data_start..].find("</pkg:binaryData>") {
                    // Binary data in XML packages is base64 encoded
                    part_xml[data_start..data_start + data_end]
                        .trim()
                        .to_string()
                } else {
                    // Skip this part but continue processing
                    start_idx = part_end;
                    continue;
                }
            } else {
                // Skip this part but continue processing
                start_idx = part_end;
                continue;
            };

            parts.push(XmlPackagePart {
                name,
                _content_type: content_type,
                data,
            });

            start_idx = part_end;
        } else {
            // No closing tag found, move to next position to avoid infinite loop
            start_idx = part_start + 1;
        }
    }

    Ok(parts)
}

/// Read a Docx from Microsoft Word XML format (single XML file with all parts)
pub fn read_xml(xml_content: &str) -> Result<Docx, ReaderError> {
    let mut docx = Docx::new();

    // Extract all parts from the XML package
    let parts = extract_xml_package_parts(xml_content)?;

    // Create a HashMap for easy lookup of parts by name
    let mut part_map: HashMap<String, String> = HashMap::new();
    for part in parts {
        part_map.insert(part.name, part.data);
    }

    // Read content types (not strictly necessary for XML format, but helps with compatibility)
    let _content_types = if let Some(content_types_data) = part_map.get("[Content_Types].xml") {
        ContentTypes::from_xml(content_types_data.as_bytes()).ok()
    } else {
        None
    };

    // Read main relationships (try both with and without leading slash)
    let rels = if let Some(rels_data) = part_map.get("_rels/.rels") {
        Rels::from_xml(rels_data.as_bytes())?
    } else if let Some(rels_data) = part_map.get("/_rels/.rels") {
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

    // Read document relationships - use Rels directly for XML packages
    let document_rels_path = document_path.replace("document.xml", "_rels/document.xml.rels");
    let document_rels = if let Some(rels_data) = part_map.get(&document_rels_path) {
        Rels::from_xml(rels_data.as_bytes())?
    } else if let Some(rels_data) = part_map.get(&format!("/{}", document_rels_path)) {
        Rels::from_xml(rels_data.as_bytes())?
    } else {
        Rels::default()
    };

    // Read themes (improved implementation with debug)
    if let Some(theme_rel) = document_rels.find_target(THEME_TYPE) {
        let base_path = document_path
            .replace("document.xml", "")
            .trim_end_matches('/')
            .to_string();
        let theme_path_str = format!("{}/{}", base_path, theme_rel.2);

        // Also try the direct path from XML package
        let direct_theme_path = if theme_path_str.starts_with("/") {
            theme_path_str.clone()
        } else {
            format!("/{}", theme_path_str)
        };

        if let Some(theme_data) = part_map
            .get(&theme_path_str)
            .or_else(|| part_map.get(&direct_theme_path))
        {
            match Theme::from_xml(theme_data.as_bytes()) {
                Ok(theme) => {
                    docx.themes.push(theme);
                }
                Err(_) => {
                    // Theme parsing failed - this is a known limitation for some XML formats
                }
            }
        }
    }

    // Read headers and footers
    let headers = read_headers_from_xml(&document_rels, &part_map, &document_path);
    let footers = read_footers_from_xml(&document_rels, &part_map, &document_path);

    // Read comments and comments extended
    let (comments, comments_extended) =
        read_comments_from_xml(&document_rels, &part_map, &document_path);

    // Read the main document (try both with and without leading slash)
    let document = if let Some(doc_data) = part_map.get(&document_path) {
        Document::from_xml(doc_data.as_bytes())?
    } else if let Some(doc_data) = part_map.get(&format!("/{}", document_path)) {
        Document::from_xml(doc_data.as_bytes())?
    } else if let Some(stripped_path) = document_path.strip_prefix("/") {
        if let Some(doc_data) = part_map.get(stripped_path) {
            Document::from_xml(doc_data.as_bytes())?
        } else {
            return Err(ReaderError::DocumentNotFoundError);
        }
    } else {
        return Err(ReaderError::DocumentNotFoundError);
    };

    docx = docx.document(document);

    // Read styles if available (improved implementation with debug)
    if let Some(styles_rel) = document_rels.find_target(STYLE_RELATIONSHIP_TYPE) {
        let base_path = document_path
            .replace("document.xml", "")
            .trim_end_matches('/')
            .to_string();
        let styles_path_str = format!("{}/{}", base_path, styles_rel.2);

        // Also try the direct path from XML package
        let direct_styles_path = if styles_path_str.starts_with("/") {
            styles_path_str.clone()
        } else {
            format!("/{}", styles_path_str)
        };

        if let Some(styles_data) = part_map
            .get(&styles_path_str)
            .or_else(|| part_map.get(&direct_styles_path))
        {
            if let Ok(styles) = Styles::from_xml(styles_data.as_bytes()) {
                docx.styles = styles;
            }
        }
    }

    // Read numbering if available (improved implementation with debug)
    if let Some(numbering_rel) = document_rels.find_target(NUMBERING_RELATIONSHIP_TYPE) {
        let base_path = document_path
            .replace("document.xml", "")
            .trim_end_matches('/')
            .to_string();
        let numbering_path_str = format!("{}/{}", base_path, numbering_rel.2);

        // Also try the direct path from XML package
        let direct_numbering_path = if numbering_path_str.starts_with("/") {
            numbering_path_str.clone()
        } else {
            format!("/{}", numbering_path_str)
        };

        if let Some(numbering_data) = part_map
            .get(&numbering_path_str)
            .or_else(|| part_map.get(&direct_numbering_path))
        {
            if let Ok(numberings) = Numberings::from_xml(numbering_data.as_bytes()) {
                docx.numberings = numberings;
            }
        }
    } else {
        // Try to find numbering.xml directly in the package if no relationship is found
        let direct_paths = ["/word/numbering.xml", "word/numbering.xml"];
        for path in &direct_paths {
            if let Some(numbering_data) = part_map.get(*path) {
                if let Ok(numberings) = Numberings::from_xml(numbering_data.as_bytes()) {
                    docx.numberings = numberings;
                    break;
                }
            }
        }
    }

    // Read settings if available (basic implementation)
    if let Some(settings_rel) = document_rels.find_target(SETTINGS_TYPE) {
        let base_path = document_path
            .replace("document.xml", "")
            .trim_end_matches('/')
            .to_string();
        let settings_path_str = format!("{}/{}", base_path, settings_rel.2);
        if let Some(settings_data) = part_map.get(&settings_path_str) {
            if let Ok(settings) = Settings::from_xml(settings_data.as_bytes()) {
                docx.settings = settings;
            }
        }
    }

    // Read web settings if available (basic implementation)
    if let Some(web_settings_rel) = document_rels.find_target(WEB_SETTINGS_TYPE) {
        let base_path = document_path
            .replace("document.xml", "")
            .trim_end_matches('/')
            .to_string();
        let web_settings_path_str = format!("{}/{}", base_path, web_settings_rel.2);
        if let Some(web_settings_data) = part_map.get(&web_settings_path_str) {
            if let Ok(web_settings) = WebSettings::from_xml(web_settings_data.as_bytes()) {
                docx.web_settings = web_settings;
            }
        }
    }

    // Assign headers
    if let Some(h) = docx.document.section_property.header_reference.clone() {
        if let Some((header, header_rels)) = headers.get(&h.id) {
            docx.document = docx.document.header(header.clone(), &h.id);
            let count = docx.document_rels.header_count + 1;
            docx.document_rels.header_count = count;
            docx.content_type = docx.content_type.add_header();

            // Read media from header if available
            docx = add_header_footer_images(docx, header_rels, &part_map, &document_path);
        }
    }
    if let Some(ref h) = docx
        .document
        .section_property
        .first_header_reference
        .clone()
    {
        if let Some((header, header_rels)) = headers.get(&h.id) {
            docx.document = docx
                .document
                .first_header_without_title_pg(header.clone(), &h.id);
            let count = docx.document_rels.header_count + 1;
            docx.document_rels.header_count = count;
            docx.content_type = docx.content_type.add_header();

            // Read media from header if available
            docx = add_header_footer_images(docx, header_rels, &part_map, &document_path);
        }
    }
    if let Some(ref h) = docx.document.section_property.even_header_reference.clone() {
        if let Some((header, header_rels)) = headers.get(&h.id) {
            docx.document = docx.document.even_header(header.clone(), &h.id);
            let count = docx.document_rels.header_count + 1;
            docx.document_rels.header_count = count;
            docx.content_type = docx.content_type.add_header();

            // Read media from header if available
            docx = add_header_footer_images(docx, header_rels, &part_map, &document_path);
        }
    }

    // Assign footers
    if let Some(f) = docx.document.section_property.footer_reference.clone() {
        if let Some((footer, footer_rels)) = footers.get(&f.id) {
            docx.document = docx.document.footer(footer.clone(), &f.id);
            let count = docx.document_rels.footer_count + 1;
            docx.document_rels.footer_count = count;
            docx.content_type = docx.content_type.add_footer();

            // Read media from footer if available
            docx = add_header_footer_images(docx, footer_rels, &part_map, &document_path);
        }
    }
    if let Some(ref f) = docx
        .document
        .section_property
        .first_footer_reference
        .clone()
    {
        if let Some((footer, footer_rels)) = footers.get(&f.id) {
            docx.document = docx
                .document
                .first_footer_without_title_pg(footer.clone(), &f.id);
            let count = docx.document_rels.footer_count + 1;
            docx.document_rels.footer_count = count;
            docx.content_type = docx.content_type.add_footer();

            // Read media from footer if available
            docx = add_header_footer_images(docx, footer_rels, &part_map, &document_path);
        }
    }
    if let Some(ref f) = docx.document.section_property.even_footer_reference.clone() {
        if let Some((footer, footer_rels)) = footers.get(&f.id) {
            docx.document = docx.document.even_footer(footer.clone(), &f.id);
            let count = docx.document_rels.footer_count + 1;
            docx.document_rels.footer_count = count;
            docx.content_type = docx.content_type.add_footer();

            // Read media from footer if available
            docx = add_header_footer_images(docx, footer_rels, &part_map, &document_path);
        }
    }

    // Store comments to paragraphs
    if !comments.inner().is_empty() {
        docx.store_comments(comments.inner());
        docx = docx.comments(comments);
        docx = docx.comments_extended(comments_extended);
    }

    // Read and add images from XML package
    let media = document_rels
        .rels
        .iter()
        .filter(|(rel_type, ..)| *rel_type == IMAGE_TYPE)
        .map(|(_, id, target)| (id.clone(), PathBuf::from(target), None))
        .collect::<Vec<_>>();

    if !media.is_empty() {
        docx = add_images_from_xml(docx, Some(media), &part_map, &document_path);
    }

    // Read and add hyperlinks - simplified for XML packages
    if let Some((id, _, target)) = document_rels.find_target(HYPERLINK_TYPE) {
        // For XML packages, we'll add the hyperlink with a default mode
        docx = docx.add_hyperlink(id, target, "External".to_string());
    }

    Ok(docx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_xml_with_numbering() {
        let xml_content = include_str!("../../../1.xml");
        let docx = read_xml(xml_content).unwrap();
        
        println!("Abstract nums count: {}", docx.numberings.abstract_nums.len());
        println!("Numberings count: {}", docx.numberings.numberings.len());
        
        // 最低限のnumberingが読み取られていることを確認
        assert!(docx.numberings.abstract_nums.len() > 0, "Abstract numberings should be read");
        assert!(docx.numberings.numberings.len() > 0, "Numberings should be read");
    }

    #[test]
    fn test_read_xml_basic() {
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

        let result = read_xml(xml_content);
        assert!(result.is_ok(), "Failed to parse XML: {:?}", result.err());

        let docx = result.unwrap();
        assert!(
            !docx.document.children.is_empty(),
            "Document should contain some content"
        );
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

    #[test]
    fn test_read_xml_with_image() {
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
      <w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture">
        <w:body>
          <w:p>
            <w:r>
              <w:drawing>
                <wp:inline distT="0" distB="0" distL="0" distR="0">
                  <wp:extent cx="1000000" cy="1000000"/>
                  <wp:docPr id="1" name="Picture 1"/>
                  <wp:cNvGraphicFramePr>
                    <a:graphicFrameLocks noChangeAspect="1"/>
                  </wp:cNvGraphicFramePr>
                  <a:graphic>
                    <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture">
                      <pic:pic>
                        <pic:nvPicPr>
                          <pic:cNvPr id="1" name="test.png"/>
                          <pic:cNvPicPr/>
                        </pic:nvPicPr>
                        <pic:blipFill>
                          <a:blip r:embed="rId1"/>
                          <a:stretch>
                            <a:fillRect/>
                          </a:stretch>
                        </pic:blipFill>
                        <pic:spPr>
                          <a:xfrm>
                            <a:off x="0" y="0"/>
                            <a:ext cx="1000000" cy="1000000"/>
                          </a:xfrm>
                          <a:prstGeom prst="rect">
                            <a:avLst/>
                          </a:prstGeom>
                        </pic:spPr>
                      </pic:pic>
                    </a:graphicData>
                  </a:graphic>
                </wp:inline>
              </w:drawing>
            </w:r>
          </w:p>
        </w:body>
      </w:document>
    </pkg:xmlData>
  </pkg:part>
  <pkg:part pkg:name="/word/_rels/document.xml.rels" pkg:contentType="application/vnd.openxmlformats-package.relationships+xml">
    <pkg:xmlData>
      <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
        <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image" Target="media/image1.png"/>
      </Relationships>
    </pkg:xmlData>
  </pkg:part>
  <pkg:part pkg:name="/word/media/image1.png" pkg:contentType="image/png">
    <pkg:binaryData>iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==</pkg:binaryData>
  </pkg:part>
</pkg:package>"#;

        let result = read_xml(xml_content);
        assert!(
            result.is_ok(),
            "Failed to parse XML with image: {:?}",
            result.err()
        );

        let docx = result.unwrap();
        assert!(
            !docx.document.children.is_empty(),
            "Document should contain some content"
        );

        // Check that the image was loaded
        assert!(!docx.images.is_empty(), "Document should contain images");
        assert_eq!(
            docx.images.len(),
            1,
            "Document should contain exactly one image"
        );

        // Verify the image data was decoded correctly (this is a 1x1 pixel PNG)
        let image = &docx.images[0];
        assert_eq!(image.1, "media/image1.png", "Image path should match");
        assert!(!image.2 .0.is_empty(), "Image data should not be empty");

        // Verify it's proper PNG data (starts with PNG magic bytes)
        assert_eq!(
            &image.2 .0[0..8],
            &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A],
            "Should be valid PNG data"
        );
    }
}
