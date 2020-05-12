use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    open!(open_pic, "pic:pic", "xmlns:pic");
    open!(open_blip_fill, "pic:blipFill");
    closed!(a_blip, "a:blip", "r:embed");
    closed!(a_src_rect, "a:srcRect");
    open!(open_a_stretch, "a:stretch");
    closed!(a_fill_rect, "a:fillRect");

    open!(open_pic_nv_pic_pr, "pic:nvPicPr");
    closed!(pic_c_nv_pr, "pic:cNvPr", "id", "name");
    open!(open_pic_c_nv_pic_pr, "pic:cNvPicPr");
    closed!(
        a_pic_locks,
        "a:picLocks",
        "noChangeAspect",
        "noChangeArrowheads"
    );

    open!(open_pic_sp_pr, "pic:spPr", "bwMode");
    open!(open_a_xfrm, "a:xfrm");
    closed!(a_off, "a:off", "x", "y");
    closed!(a_ext, "a:ext", "cx", "cy");
    open!(open_a_prst_geom, "a:prstGeom", "prst");
    closed!(a_av_lst, "a:avLst");
}
