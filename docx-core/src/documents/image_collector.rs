use crate::{
    DeleteChild, DrawingData, InsertChild, Paragraph, ParagraphChild, RunChild,
    StructuredDataTagChild, Table, TableCellContent, TableChild, TableRowChild, TocContent,
};

pub(crate) fn collect_images_from_paragraph(
    paragraph: &mut Paragraph,
    images: &mut Vec<(String, String)>,
    image_bufs: &mut Vec<(String, Vec<u8>)>,
    id_prefix: Option<&str>,
) {
    for child in &mut paragraph.children {
        if let ParagraphChild::Run(run) = child {
            for child in &mut run.children {
                if let RunChild::Drawing(d) = child {
                    if let Some(DrawingData::Pic(pic)) = &mut d.data {
                        let b = std::mem::take(&mut pic.image);
                        let buf = image_bufs.iter().find(|x| x.0 == pic.id || x.1 == b);
                        let pic_id = if let Some(prefix) = id_prefix {
                            format!("{}{}", pic.id, prefix)
                        } else {
                            pic.id.clone()
                        };
                        if buf.as_ref().is_none() {
                            images.push((
                                pic_id.clone(),
                                // For now only png supported
                                format!("media/{}.png", pic.id),
                            ));
                            image_bufs.push((pic_id.clone(), b));
                            pic.id = pic_id;
                        } else {
                            pic.id = buf.unwrap().0.clone();
                        }
                    }
                }
            }
        } else if let ParagraphChild::Insert(ins) = child {
            for child in &mut ins.children {
                match child {
                    InsertChild::Run(run) => {
                        for child in &mut run.children {
                            if let RunChild::Drawing(d) = child {
                                if let Some(DrawingData::Pic(pic)) = &mut d.data {
                                    images.push((
                                        pic.id.clone(),
                                        // For now only png supported
                                        format!("media/{}.png", pic.id),
                                    ));
                                    let b = std::mem::take(&mut pic.image);
                                    image_bufs.push((pic.id.clone(), b));
                                }
                            }
                        }
                    }
                    InsertChild::Delete(del) => {
                        for d in &mut del.children {
                            if let DeleteChild::Run(run) = d {
                                for child in &mut run.children {
                                    if let RunChild::Drawing(d) = child {
                                        if let Some(DrawingData::Pic(pic)) = &mut d.data {
                                            images.push((
                                                pic.id.clone(),
                                                // For now only png supported
                                                format!("media/{}.png", pic.id),
                                            ));
                                            let b = std::mem::take(&mut pic.image);
                                            image_bufs.push((pic.id.clone(), b));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        } else if let ParagraphChild::Delete(del) = child {
            for d in &mut del.children {
                if let DeleteChild::Run(run) = d {
                    for child in &mut run.children {
                        if let RunChild::Drawing(d) = child {
                            if let Some(DrawingData::Pic(pic)) = &mut d.data {
                                images.push((
                                    pic.id.clone(),
                                    // For now only png supported
                                    format!("media/{}.png", pic.id),
                                ));
                                let b = std::mem::take(&mut pic.image);
                                image_bufs.push((pic.id.clone(), b));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn collect_images_from_table(
    table: &mut Table,
    images: &mut Vec<(String, String)>,
    image_bufs: &mut Vec<(String, Vec<u8>)>,
    id_prefix: Option<&str>,
) {
    for TableChild::TableRow(row) in &mut table.rows {
        for TableRowChild::TableCell(cell) in &mut row.cells {
            for content in &mut cell.children {
                match content {
                    TableCellContent::Paragraph(paragraph) => {
                        collect_images_from_paragraph(paragraph, images, image_bufs, id_prefix);
                    }
                    TableCellContent::Table(table) => {
                        collect_images_from_table(table, images, image_bufs, id_prefix)
                    }
                    TableCellContent::StructuredDataTag(tag) => {
                        for child in &mut tag.children {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph, images, image_bufs, id_prefix,
                                );
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                collect_images_from_table(table, images, image_bufs, id_prefix);
                            }
                        }
                    }
                    TableCellContent::TableOfContents(t) => {
                        for child in &mut t.before_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph, images, image_bufs, id_prefix,
                                );
                            }
                            if let TocContent::Table(table) = child {
                                collect_images_from_table(table, images, image_bufs, id_prefix);
                            }
                        }

                        for child in &mut t.after_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph, images, image_bufs, id_prefix,
                                );
                            }
                            if let TocContent::Table(table) = child {
                                collect_images_from_table(table, images, image_bufs, id_prefix);
                            }
                        }
                    }
                }
            }
        }
    }
}
