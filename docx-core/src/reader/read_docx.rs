use super::header_or_footer_rels::{read_header_or_footer_rels, ReadHeaderOrFooterRels};
use super::namespace::*;
use super::*;

use zip::ZipArchive;

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
                if !comments.is_empty() && !comments_extended.children.is_empty() {
                    let mut comment_id_by_paragraph: HashMap<String, usize> = HashMap::new();
                    for comment in &comments {
                        for child in &comment.children {
                            if let CommentChild::Paragraph(p) = child {
                                comment_id_by_paragraph.insert(p.id.clone(), comment.id);
                            }
                        }
                    }

                    let mut parent_paragraph_by_paragraph: HashMap<String, String> = HashMap::new();
                    for extended in &comments_extended.children {
                        if let Some(parent_paragraph_id) = extended.parent_paragraph_id.as_deref() {
                            parent_paragraph_by_paragraph.insert(
                                extended.paragraph_id.clone(),
                                parent_paragraph_id.to_string(),
                            );
                        }
                    }

                    for comment in &mut comments {
                        let mut parent_comment_id = None;
                        for child in &comment.children {
                            if let CommentChild::Paragraph(p) = child {
                                if let Some(parent_paragraph_id) =
                                    parent_paragraph_by_paragraph.get(&p.id)
                                {
                                    parent_comment_id =
                                        comment_id_by_paragraph.get(parent_paragraph_id).copied();
                                    if parent_comment_id.is_some() {
                                        break;
                                    }
                                }
                            }
                        }
                        comment.parent_comment_id = parent_comment_id;
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
