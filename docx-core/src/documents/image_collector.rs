use crate::{
    DeleteChild, DrawingData, InsertChild, Paragraph, ParagraphChild, Pic, RunChild,
    StructuredDataTagChild, Table, TableCellContent, TableChild, TableRowChild, TocContent,
};
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

#[derive(Default)]
pub(crate) struct ImageDeduplicator {
    by_id: HashMap<String, usize>,
    by_hash: HashMap<u64, Vec<usize>>,
}

fn collect_pic(
    pic: &mut Pic,
    images: &mut Vec<(String, String)>,
    image_bufs: &mut Vec<(String, Vec<u8>)>,
    id_prefix: Option<&str>,
    deduplicator: &mut ImageDeduplicator,
) {
    let image = std::mem::take(&mut pic.image);

    if let Some(index) = deduplicator.by_id.get(&pic.id).copied() {
        pic.id = image_bufs[index].0.clone();
        return;
    }

    let mut hasher = DefaultHasher::new();
    image.hash(&mut hasher);
    let hash = hasher.finish();
    if let Some(index) = deduplicator.by_hash.get(&hash).and_then(|candidates| {
        candidates
            .iter()
            .copied()
            .find(|index| image_bufs[*index].1 == image)
    }) {
        pic.id = image_bufs[index].0.clone();
        return;
    }

    let pic_id = if let Some(prefix) = id_prefix {
        format!("{prefix}{}", pic.id)
    } else {
        pic.id.clone()
    };
    images.push((pic_id.clone(), format!("media/{pic_id}.png")));
    let index = image_bufs.len();
    image_bufs.push((pic_id.clone(), image));
    deduplicator.by_id.insert(pic_id.clone(), index);
    deduplicator.by_hash.entry(hash).or_default().push(index);
    pic.id = pic_id;
}

pub(crate) fn collect_images_from_paragraph(
    paragraph: &mut Paragraph,
    images: &mut Vec<(String, String)>,
    image_bufs: &mut Vec<(String, Vec<u8>)>,
    id_prefix: Option<&str>,
    deduplicator: &mut ImageDeduplicator,
) {
    for child in &mut paragraph.children {
        if let ParagraphChild::Run(run) = child {
            for child in &mut run.children {
                if let RunChild::Drawing(d) = child {
                    if let Some(DrawingData::Pic(pic)) = &mut d.data {
                        collect_pic(pic, images, image_bufs, id_prefix, deduplicator);
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
                                    collect_pic(pic, images, image_bufs, id_prefix, deduplicator);
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
                                            collect_pic(
                                                pic,
                                                images,
                                                image_bufs,
                                                id_prefix,
                                                deduplicator,
                                            );
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
                                collect_pic(pic, images, image_bufs, id_prefix, deduplicator);
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
    deduplicator: &mut ImageDeduplicator,
) {
    for TableChild::TableRow(row) in &mut table.rows {
        for TableRowChild::TableCell(cell) in &mut row.cells {
            for content in &mut cell.children {
                match content {
                    TableCellContent::Paragraph(paragraph) => {
                        collect_images_from_paragraph(
                            paragraph,
                            images,
                            image_bufs,
                            id_prefix,
                            deduplicator,
                        );
                    }
                    TableCellContent::Table(table) => collect_images_from_table(
                        table,
                        images,
                        image_bufs,
                        id_prefix,
                        deduplicator,
                    ),
                    TableCellContent::StructuredDataTag(tag) => {
                        for child in &mut tag.children {
                            if let StructuredDataTagChild::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph,
                                    images,
                                    image_bufs,
                                    id_prefix,
                                    deduplicator,
                                );
                            }
                            if let StructuredDataTagChild::Table(table) = child {
                                collect_images_from_table(
                                    table,
                                    images,
                                    image_bufs,
                                    id_prefix,
                                    deduplicator,
                                );
                            }
                        }
                    }
                    TableCellContent::TableOfContents(t) => {
                        for child in &mut t.before_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph,
                                    images,
                                    image_bufs,
                                    id_prefix,
                                    deduplicator,
                                );
                            }
                            if let TocContent::Table(table) = child {
                                collect_images_from_table(
                                    table,
                                    images,
                                    image_bufs,
                                    id_prefix,
                                    deduplicator,
                                );
                            }
                        }

                        for child in &mut t.after_contents {
                            if let TocContent::Paragraph(paragraph) = child {
                                collect_images_from_paragraph(
                                    paragraph,
                                    images,
                                    image_bufs,
                                    id_prefix,
                                    deduplicator,
                                );
                            }
                            if let TocContent::Table(table) = child {
                                collect_images_from_table(
                                    table,
                                    images,
                                    image_bufs,
                                    id_prefix,
                                    deduplicator,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Run;

    #[test]
    fn deduplicates_identical_images_across_paragraphs() {
        let image = vec![1, 2, 3, 4, 5];
        let mut first = Paragraph::new().add_run(Run::new().add_image(Pic::new_with_dimensions(
            image.clone(),
            1,
            1,
        )));
        let mut second =
            Paragraph::new().add_run(Run::new().add_image(Pic::new_with_dimensions(image, 1, 1)));
        let mut images = Vec::new();
        let mut image_bufs = Vec::new();
        let mut deduplicator = ImageDeduplicator::default();

        collect_images_from_paragraph(
            &mut first,
            &mut images,
            &mut image_bufs,
            None,
            &mut deduplicator,
        );
        collect_images_from_paragraph(
            &mut second,
            &mut images,
            &mut image_bufs,
            None,
            &mut deduplicator,
        );

        assert_eq!(images.len(), 1);
        assert_eq!(image_bufs.len(), 1);
    }
}
