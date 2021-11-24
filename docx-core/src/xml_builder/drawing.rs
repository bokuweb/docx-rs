use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    open!(
        open_wp_inline,
        "wp:inline",
        "distT",
        "distB",
        "distL",
        "distR"
    );

    open!(
        open_wp_anchor,
        "wp:anchor",
        "distT",
        "distB",
        "distL",
        "distR",
        "simplePos",
        "allowOverlap",
        "behindDoc",
        "locked",
        "layoutInCell",
        "relativeHeight"
    );

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

    closed!(simple_pos, "wp:simplePos", "x", "y");
    open!(open_position_h, "wp:positionH", "relativeFrom");
    open!(open_position_v, "wp:positionV", "relativeFrom");
    closed_with_child!(pos_offset, "wp:posOffset");
    closed!(wrap_none, "wp:wrapNone");
}
