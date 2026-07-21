//! Shared traversal for package-relevant nodes in a DOCX document tree.
//!
//! Document content can be nested under tables, sections, structured data
//! tags, hyperlinks, and tracked changes. Centralizing that recursion keeps
//! dependency collectors complete when new package metadata is added.

use crate::{
    Delete, DeleteChild, Document, DocumentChild, DrawingData, Footer, FooterChild, Header,
    HeaderChild, Hyperlink, Insert, InsertChild, MoveFrom, MoveFromChild, MoveTo, MoveToChild,
    Paragraph, ParagraphChild, Pic, Run, RunChild, Section, SectionChild, StructuredDataTag,
    StructuredDataTagChild, Table, TableCellContent, TableChild, TableOfContents, TableRowChild,
};

/// Receives package-relevant nodes while the document tree is traversed.
///
/// Keeping traversal in one module prevents media and relationship collectors
/// from silently disagreeing about nested containers such as tables, sections,
/// structured data tags, hyperlinks, and tracked changes. Default hooks make
/// it possible to collect only the data needed by a particular package part.
pub(crate) trait DocumentTreeVisitor {
    /// Visits a paragraph before its children are traversed.
    fn visit_paragraph(&mut self, _paragraph: &mut Paragraph) {}

    /// Visits a run before its children are traversed.
    fn visit_run(&mut self, _run: &mut Run) {}

    /// Visits an embedded DrawingML picture.
    fn visit_picture(&mut self, _picture: &mut Pic) {}

    /// Visits a footnote reference stored in a run.
    fn visit_footnote_reference(&mut self, _reference: &crate::FootnoteReference) {}
}

/// Traverses the main document body without crossing into header/footer parts.
///
/// Header and footer relationships have their own scopes, so callers must walk
/// those parts separately with [`visit_header`] and [`visit_footer`].
pub(crate) fn visit_document(document: &mut Document, visitor: &mut impl DocumentTreeVisitor) {
    for child in &mut document.children {
        visit_document_child(child, visitor);
    }
}

/// Traverses every package-relevant node in one header part.
pub(crate) fn visit_header(header: &mut Header, visitor: &mut impl DocumentTreeVisitor) {
    for child in &mut header.children {
        match child {
            HeaderChild::Paragraph(paragraph) => visit_paragraph(paragraph, visitor),
            HeaderChild::Table(table) => visit_table(table, visitor),
            HeaderChild::StructuredDataTag(tag) => visit_structured_data_tag(tag, visitor),
        }
    }
}

/// Traverses every package-relevant node in one footer part.
pub(crate) fn visit_footer(footer: &mut Footer, visitor: &mut impl DocumentTreeVisitor) {
    for child in &mut footer.children {
        match child {
            FooterChild::Paragraph(paragraph) => visit_paragraph(paragraph, visitor),
            FooterChild::Table(table) => visit_table(table, visitor),
            FooterChild::StructuredDataTag(tag) => visit_structured_data_tag(tag, visitor),
        }
    }
}

fn visit_document_child(child: &mut DocumentChild, visitor: &mut impl DocumentTreeVisitor) {
    match child {
        DocumentChild::Paragraph(paragraph) => visit_paragraph(paragraph, visitor),
        DocumentChild::Table(table) => visit_table(table, visitor),
        DocumentChild::StructuredDataTag(tag) => visit_structured_data_tag(tag, visitor),
        DocumentChild::TableOfContents(toc) => visit_table_of_contents(toc, visitor),
        DocumentChild::Section(section) => visit_section(section, visitor),
        DocumentChild::BookmarkStart(_)
        | DocumentChild::BookmarkEnd(_)
        | DocumentChild::CommentStart(_)
        | DocumentChild::CommentEnd(_) => {}
    }
}

fn visit_section(section: &mut Section, visitor: &mut impl DocumentTreeVisitor) {
    for child in &mut section.children {
        match child {
            SectionChild::Paragraph(paragraph) => visit_paragraph(paragraph, visitor),
            SectionChild::Table(table) => visit_table(table, visitor),
            SectionChild::StructuredDataTag(tag) => visit_structured_data_tag(tag, visitor),
            SectionChild::TableOfContents(toc) => visit_table_of_contents(toc, visitor),
            SectionChild::BookmarkStart(_)
            | SectionChild::BookmarkEnd(_)
            | SectionChild::CommentStart(_)
            | SectionChild::CommentEnd(_) => {}
        }
    }
}

fn visit_table(table: &mut Table, visitor: &mut impl DocumentTreeVisitor) {
    for TableChild::TableRow(row) in &mut table.rows {
        for TableRowChild::TableCell(cell) in &mut row.cells {
            for content in &mut cell.children {
                match content {
                    TableCellContent::Paragraph(paragraph) => visit_paragraph(paragraph, visitor),
                    TableCellContent::Table(table) => visit_table(table, visitor),
                    TableCellContent::StructuredDataTag(tag) => {
                        visit_structured_data_tag(tag, visitor)
                    }
                    TableCellContent::TableOfContents(toc) => visit_table_of_contents(toc, visitor),
                }
            }
        }
    }
}

fn visit_table_of_contents(toc: &mut TableOfContents, visitor: &mut impl DocumentTreeVisitor) {
    for content in toc
        .before_contents
        .iter_mut()
        .chain(toc.after_contents.iter_mut())
    {
        match content {
            crate::TocContent::Paragraph(paragraph) => visit_paragraph(paragraph, visitor),
            crate::TocContent::Table(table) => visit_table(table, visitor),
        }
    }
}

fn visit_structured_data_tag(tag: &mut StructuredDataTag, visitor: &mut impl DocumentTreeVisitor) {
    for child in &mut tag.children {
        match child {
            StructuredDataTagChild::Run(run) => visit_run(run, visitor),
            StructuredDataTagChild::Paragraph(paragraph) => visit_paragraph(paragraph, visitor),
            StructuredDataTagChild::Table(table) => visit_table(table, visitor),
            StructuredDataTagChild::StructuredDataTag(tag) => {
                visit_structured_data_tag(tag, visitor)
            }
            StructuredDataTagChild::BookmarkStart(_)
            | StructuredDataTagChild::BookmarkEnd(_)
            | StructuredDataTagChild::CommentStart(_)
            | StructuredDataTagChild::CommentEnd(_) => {}
        }
    }
}

fn visit_paragraph(paragraph: &mut Paragraph, visitor: &mut impl DocumentTreeVisitor) {
    visitor.visit_paragraph(paragraph);
    visit_paragraph_children(&mut paragraph.children, visitor);
}

fn visit_paragraph_children(
    children: &mut [ParagraphChild],
    visitor: &mut impl DocumentTreeVisitor,
) {
    for child in children {
        match child {
            ParagraphChild::Run(run) => visit_run(run, visitor),
            ParagraphChild::Insert(insert) => visit_insert(insert, visitor),
            ParagraphChild::Delete(delete) => visit_delete(delete, visitor),
            ParagraphChild::MoveFrom(moved) => visit_move_from(moved, visitor),
            ParagraphChild::MoveTo(moved) => visit_move_to(moved, visitor),
            ParagraphChild::Hyperlink(link) => visit_hyperlink(link, visitor),
            ParagraphChild::StructuredDataTag(tag) => visit_structured_data_tag(tag, visitor),
            ParagraphChild::BookmarkStart(_)
            | ParagraphChild::BookmarkEnd(_)
            | ParagraphChild::CommentStart(_)
            | ParagraphChild::CommentEnd(_)
            | ParagraphChild::PageNum(_)
            | ParagraphChild::NumPages(_) => {}
        }
    }
}

fn visit_hyperlink(link: &mut Hyperlink, visitor: &mut impl DocumentTreeVisitor) {
    visit_paragraph_children(&mut link.children, visitor);
}

fn visit_insert(insert: &mut Insert, visitor: &mut impl DocumentTreeVisitor) {
    for child in &mut insert.children {
        match child {
            InsertChild::Run(run) => visit_run(run, visitor),
            InsertChild::Delete(delete) => visit_delete(delete, visitor),
            InsertChild::CommentStart(_) | InsertChild::CommentEnd(_) => {}
        }
    }
}

fn visit_delete(delete: &mut Delete, visitor: &mut impl DocumentTreeVisitor) {
    for child in &mut delete.children {
        if let DeleteChild::Run(run) = child {
            visit_run(run, visitor);
        }
    }
}

fn visit_move_from(moved: &mut MoveFrom, visitor: &mut impl DocumentTreeVisitor) {
    for child in &mut moved.children {
        if let MoveFromChild::Run(run) = child {
            visit_run(run, visitor);
        }
    }
}

fn visit_move_to(moved: &mut MoveTo, visitor: &mut impl DocumentTreeVisitor) {
    for child in &mut moved.children {
        match child {
            MoveToChild::Run(run) => visit_run(run, visitor),
            MoveToChild::Delete(delete) => visit_delete(delete, visitor),
            MoveToChild::CommentStart(_) | MoveToChild::CommentEnd(_) => {}
        }
    }
}

fn visit_run(run: &mut Run, visitor: &mut impl DocumentTreeVisitor) {
    visitor.visit_run(run);
    for child in &mut run.children {
        match child {
            RunChild::Drawing(drawing) => {
                if let Some(DrawingData::Pic(picture)) = &mut drawing.data {
                    visitor.visit_picture(picture);
                }
            }
            RunChild::FootnoteReference(reference) => visitor.visit_footnote_reference(reference),
            RunChild::Text(_)
            | RunChild::Sym(_)
            | RunChild::DeleteText(_)
            | RunChild::Tab(_)
            | RunChild::PTab(_)
            | RunChild::Break(_)
            | RunChild::CarriageReturn(_)
            | RunChild::Shape(_)
            | RunChild::CommentStart(_)
            | RunChild::CommentEnd(_)
            | RunChild::FieldChar(_)
            | RunChild::InstrText(_)
            | RunChild::DeleteInstrText(_)
            | RunChild::InstrTextString(_)
            | RunChild::Shading(_) => {}
        }
    }
}
