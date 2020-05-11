use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    open!(open_pic, "pic:pic", "xmlns:pic");
    open!(open_blip_fill, "pic:blipFill");
    closed!(a_blip, "a:blip", "r:embed", "cstate");
    closed!(a_src_rect, "a:srcRect");
    open!(open_a_stretch, "a:stretch");
    closed!(a_fill_rect, "a:fillRect");

    open!(open_pic_nv_piv_pr, "pic:nvPicPr");
    open!(open_pic_sp_pr, "pic:spPr", "bwMode");
}
