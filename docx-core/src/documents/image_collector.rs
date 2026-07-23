//! Collects image relationships, physical media, and footnotes for packaging.
//!
//! OPC relationships belong to individual XML parts, but media files belong
//! to the whole package. This module models those scopes separately so shared
//! image bytes are written once without losing any part-local relationship.

use std::collections::{HashMap, HashSet};

use super::document_tree::{visit_document, visit_footer, visit_header, DocumentTreeVisitor};
use crate::{
    Document, Footer, Footnote, FootnoteReference, Header, ImageIdAndBuf, ImageIdAndPath, Pic,
};

/// Stores physical media once for the entire OPC package.
///
/// Relationship IDs are scoped to individual XML parts, while files under
/// `word/media` are package-global. Keeping those concepts separate allows a
/// body, header, and footer to reference the same bytes without writing three
/// copies or dropping a relationship from one of the parts.
#[derive(Default)]
pub(crate) struct MediaRegistry {
    media: Vec<ImageIdAndBuf>,
    media_by_id: HashMap<String, usize>,
    by_fingerprint: HashMap<(usize, u32), Vec<usize>>,
    next_suffix_by_id: HashMap<String, usize>,
}

impl MediaRegistry {
    /// Registers physical bytes and returns their stable registry index.
    ///
    /// Part collectors use the numeric index as their deduplication key. This
    /// avoids cloning and hashing a `media/<id>.png` target for every repeated
    /// picture while keeping the package-visible media ID encapsulated here.
    /// Reused picture IDs are checked before calculating a content fingerprint;
    /// byte equality still protects callers that reuse an ID for new content.
    fn register(&mut self, preferred_id: &str, bytes: Vec<u8>) -> usize {
        if let Some(index) = self.media_by_id.get(preferred_id).copied() {
            if self.media[index].1 == bytes {
                return index;
            }
        }

        let fingerprint = (bytes.len(), crc32fast::hash(&bytes));
        if let Some(index) = self
            .by_fingerprint
            .get(&fingerprint)
            .and_then(|candidates| {
                candidates
                    .iter()
                    .copied()
                    .find(|index| self.media[*index].1 == bytes)
            })
        {
            return index;
        }

        let media_id = self.unique_media_id(preferred_id);
        let index = self.media.len();
        self.media.push((media_id.clone(), bytes));
        self.media_by_id.insert(media_id, index);
        self.by_fingerprint
            .entry(fingerprint)
            .or_default()
            .push(index);
        index
    }

    /// Returns the package-visible ID assigned to registered media.
    fn media_id(&self, index: usize) -> &str {
        &self.media[index].0
    }

    /// Consumes the registry and returns files ready for ZIP packaging.
    pub(crate) fn into_media(self) -> Vec<ImageIdAndBuf> {
        self.media
    }

    /// Returns a free media ID without restarting collision scans at `_2`.
    ///
    /// Callers can reuse a preferred ID for different bytes. Remembering the
    /// next suffix keeps a run of such collisions linear while still checking
    /// explicitly registered suffixed IDs.
    fn unique_media_id(&mut self, preferred_id: &str) -> String {
        if !self.media_by_id.contains_key(preferred_id) {
            return preferred_id.to_owned();
        }

        let (media_by_id, next_suffix_by_id) = (&self.media_by_id, &mut self.next_suffix_by_id);
        let suffix = next_suffix_by_id
            .entry(preferred_id.to_owned())
            .or_insert(2);

        loop {
            let candidate = format!("{preferred_id}_{suffix}");
            *suffix = suffix
                .checked_add(1)
                .expect("the media ID suffix space should not be exhausted");
            if !media_by_id.contains_key(&candidate) {
                return candidate;
            }
        }
    }
}

/// Data collected from one relationship scope during a tree walk.
pub(crate) struct CollectedPart {
    pub(crate) relationships: Vec<ImageIdAndPath>,
    pub(crate) footnotes: Vec<Footnote>,
}

/// Collects media relationships and footnotes while visiting one XML part.
///
/// A fresh collector is created for each relationship scope, but every
/// collector shares one [`MediaRegistry`]. This is the key distinction that
/// preserves per-part relationships while deduplicating physical media.
struct PackagePartCollector<'a> {
    registry: &'a mut MediaRegistry,
    relationship_prefix: Option<&'a str>,
    relationships: Vec<ImageIdAndPath>,
    relationship_ids: HashSet<String>,
    /// Maps media to the owning relationship entry, avoiding another owned ID.
    relationships_by_media: HashMap<usize, usize>,
    footnotes: Vec<Footnote>,
}

impl<'a> PackagePartCollector<'a> {
    fn new(registry: &'a mut MediaRegistry, relationship_prefix: Option<&'a str>) -> Self {
        Self {
            registry,
            relationship_prefix,
            relationships: Vec::new(),
            relationship_ids: HashSet::new(),
            relationships_by_media: HashMap::new(),
            footnotes: Vec::new(),
        }
    }

    fn finish(self) -> CollectedPart {
        CollectedPart {
            relationships: self.relationships,
            footnotes: self.footnotes,
        }
    }

    fn relationship_id(&self, picture_id: &str) -> String {
        match self.relationship_prefix {
            Some(prefix) => format!("{prefix}{picture_id}"),
            None => picture_id.to_owned(),
        }
    }

    /// Returns an ID that is unique inside this part's relationship scope.
    fn unique_relationship_id(&self, preferred_id: &str) -> String {
        match self.relationship_ids.contains(preferred_id) {
            false => preferred_id.to_owned(),
            true => (2usize..)
                .map(|suffix| format!("{preferred_id}_{suffix}"))
                .find(|candidate| !self.relationship_ids.contains(candidate))
                .expect("the relationship ID space should not be exhausted"),
        }
    }
}

impl DocumentTreeVisitor for PackagePartCollector<'_> {
    fn visit_picture(&mut self, picture: &mut Pic) {
        let preferred_relationship_id = self.relationship_id(&picture.id);
        let media_index = self.registry.register(
            &preferred_relationship_id,
            std::mem::take(&mut picture.image),
        );

        if let Some(relationship_index) = self.relationships_by_media.get(&media_index) {
            picture
                .id
                .clone_from(&self.relationships[*relationship_index].0);
            return;
        }

        let relationship_id = self.unique_relationship_id(&preferred_relationship_id);
        let target = format!("media/{}.png", self.registry.media_id(media_index));
        let relationship_index = self.relationships.len();
        self.relationship_ids.insert(relationship_id.clone());
        self.relationships_by_media
            .insert(media_index, relationship_index);
        picture.id.clone_from(&relationship_id);
        self.relationships.push((relationship_id, target));
    }

    fn visit_footnote_reference(&mut self, reference: &FootnoteReference) {
        self.footnotes.push(reference.into());
    }
}

/// Collects package data from the complete main document tree in one pass.
pub(crate) fn collect_document_part(
    document: &mut Document,
    registry: &mut MediaRegistry,
) -> CollectedPart {
    let mut collector = PackagePartCollector::new(registry, None);
    visit_document(document, &mut collector);
    collector.finish()
}

/// Collects footnotes without consuming image buffers from the document tree.
///
/// This separate visitor keeps [`crate::Docx::collect_footnotes`] compatible
/// for callers that invoke it directly. Package preparation uses
/// [`collect_document_part`] so images and footnotes are normally gathered in
/// one traversal.
pub(crate) fn collect_document_footnotes(document: &mut Document) -> Vec<Footnote> {
    #[derive(Default)]
    struct FootnoteCollector {
        footnotes: Vec<Footnote>,
    }

    impl DocumentTreeVisitor for FootnoteCollector {
        fn visit_footnote_reference(&mut self, reference: &FootnoteReference) {
            self.footnotes.push(reference.into());
        }
    }

    let mut collector = FootnoteCollector::default();
    visit_document(document, &mut collector);
    collector.footnotes
}

/// Collects package data from one header relationship scope.
pub(crate) fn collect_header_part(
    header: &mut Header,
    registry: &mut MediaRegistry,
) -> CollectedPart {
    let mut collector = PackagePartCollector::new(registry, Some("header"));
    visit_header(header, &mut collector);
    collector.finish()
}

/// Collects package data from one footer relationship scope.
pub(crate) fn collect_footer_part(
    footer: &mut Footer,
    registry: &mut MediaRegistry,
) -> CollectedPart {
    let mut collector = PackagePartCollector::new(registry, Some("footer"));
    visit_footer(footer, &mut collector);
    collector.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_reuses_the_stable_index_for_identical_media() {
        let bytes = vec![1, 2, 3, 4];
        let mut registry = MediaRegistry::default();

        let first = registry.register("first", bytes.clone());
        let second = registry.register("second", bytes);

        assert_eq!(first, second);
        assert_eq!(registry.media.len(), 1);
    }

    #[test]
    fn registry_keeps_distinct_bytes_that_reuse_an_id() {
        let mut registry = MediaRegistry::default();

        registry.register("shared_2", vec![0]);
        let first = registry.register("shared", vec![1, 2, 3]);
        let second = registry.register("shared", vec![4, 5, 6]);
        let third = registry.register("shared", vec![7, 8, 9]);

        assert_ne!(first, second);
        assert_ne!(second, third);
        assert_eq!(registry.media.len(), 4);
        assert_ne!(registry.media[first].0, registry.media[second].0);
        assert_eq!(registry.media[first].0, "shared");
        assert_eq!(registry.media[second].0, "shared_3");
        assert_eq!(registry.media[third].0, "shared_4");
    }

    #[test]
    fn registry_deduplicates_identical_media_but_keeps_part_relationships() {
        let bytes = vec![1, 2, 3, 4, 5];
        let mut first =
            Header::new().add_paragraph(crate::Paragraph::new().add_run(
                crate::Run::new().add_image(Pic::new_with_dimensions(bytes.clone(), 1, 1)),
            ));
        let mut second = Header::new().add_paragraph(
            crate::Paragraph::new()
                .add_run(crate::Run::new().add_image(Pic::new_with_dimensions(bytes, 1, 1))),
        );
        let mut registry = MediaRegistry::default();

        let first = collect_header_part(&mut first, &mut registry);
        let second = collect_header_part(&mut second, &mut registry);

        assert_eq!(registry.media.len(), 1);
        assert_eq!(first.relationships.len(), 1);
        assert_eq!(second.relationships.len(), 1);
        assert_eq!(first.relationships[0].1, second.relationships[0].1);
    }

    #[test]
    fn identical_media_reuses_one_relationship_within_a_part() {
        let bytes = vec![9, 8, 7, 6];
        let mut first = Pic::new_with_dimensions(bytes.clone(), 1, 1);
        first.id = "first".to_owned();
        let mut second = Pic::new_with_dimensions(bytes, 1, 1);
        second.id = "second".to_owned();
        let mut document = Document::new()
            .add_paragraph(crate::Paragraph::new().add_run(crate::Run::new().add_image(first)))
            .add_paragraph(crate::Paragraph::new().add_run(crate::Run::new().add_image(second)));
        let mut registry = MediaRegistry::default();

        let part = collect_document_part(&mut document, &mut registry);

        assert_eq!(registry.media.len(), 1);
        assert_eq!(part.relationships.len(), 1);
    }
}
