use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    open!(open_wp_inline, "wp:inline", "distT", "distB", "distL", "distR");
    // TODO: Add some parameters
    open!(open_wp_anchor, "wp:inline", "distT", "distB", "distL", "distR");

    open!(open_a_graphic, "a:graphic", "xmlns:a");
    open!(open_a_graphic_data, "a:graphicData", "uri");
    closed!(wp_extent, "wp:extent", "cx", "cy");
    closed!(wp_effect_extent, "wp:effectExtent", "b", "l", "r", "t");
    closed!(wp_doc_pr, "wp:docPr", "id", "name");
    open!(open_wp_c_nv_graphic_frame_pr, "wp:cNvGraphicFramePr");
    closed!(
        a_graphic_frame_locks,
        "a:graphicFrameLocks",
        "xmlns:a",
        "noChangeAspect"
    );
}
