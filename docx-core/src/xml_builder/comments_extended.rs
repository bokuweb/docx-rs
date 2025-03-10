use super::XMLBuilder;
use super::XmlEvent;
use std::io::Write;
use xml::writer::Result;

impl<W: Write> XMLBuilder<W> {
    pub(crate) fn open_comments_extended(self) -> Result<Self> {
        self.write(
            XmlEvent::start_element("w15:commentsEx")
                .attr(
                    "xmlns:wpc",
                    "http://schemas.microsoft.com/office/word/2010/wordprocessingCanvas",
                )
                .attr(
                    "xmlns:cx",
                    "http://schemas.microsoft.com/office/drawing/2014/chartex",
                )
                .attr(
                    "xmlns:cx1",
                    "http://schemas.microsoft.com/office/drawing/2015/9/8/chartex",
                )
                .attr(
                    "xmlns:cx2",
                    "http://schemas.microsoft.com/office/drawing/2015/10/21/chartex",
                )
                .attr(
                    "xmlns:cx3",
                    "http://schemas.microsoft.com/office/drawing/2016/5/9/chartex",
                )
                .attr(
                    "xmlns:cx4",
                    "http://schemas.microsoft.com/office/drawing/2016/5/10/chartex",
                )
                .attr(
                    "xmlns:cx5",
                    "http://schemas.microsoft.com/office/drawing/2016/5/11/chartex",
                )
                .attr(
                    "xmlns:cx6",
                    "http://schemas.microsoft.com/office/drawing/2016/5/12/chartex",
                )
                .attr(
                    "xmlns:cx7",
                    "http://schemas.microsoft.com/office/drawing/2016/5/13/chartex",
                )
                .attr(
                    "xmlns:cx8",
                    "http://schemas.microsoft.com/office/drawing/2016/5/14/chartex",
                )
                .attr(
                    "xmlns:mc",
                    "http://schemas.openxmlformats.org/markup-compatibility/2006",
                )
                .attr(
                    "xmlns:aink",
                    "http://schemas.microsoft.com/office/drawing/2016/ink",
                )
                .attr(
                    "xmlns:am3d",
                    "http://schemas.microsoft.com/office/drawing/2017/model3d",
                )
                .attr("xmlns:o", "urn:schemas-microsoft-com:office:office")
                .attr(
                    "xmlns:r",
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                )
                .attr(
                    "xmlns:m",
                    "http://schemas.openxmlformats.org/officeDocument/2006/math",
                )
                .attr("xmlns:v", "urn:schemas-microsoft-com:vml")
                .attr(
                    "xmlns:wp14",
                    "http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing",
                )
                .attr(
                    "xmlns:wp",
                    "http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing",
                )
                .attr("xmlns:w10", "urn:schemas-microsoft-com:office:word")
                .attr(
                    "xmlns:w",
                    "http://schemas.openxmlformats.org/wordprocessingml/2006/main",
                )
                .attr(
                    "xmlns:w14",
                    "http://schemas.microsoft.com/office/word/2010/wordml",
                )
                .attr(
                    "xmlns:w15",
                    "http://schemas.microsoft.com/office/word/2012/wordml",
                )
                .attr(
                    "xmlns:w16cex",
                    "http://schemas.microsoft.com/office/word/2018/wordml/cex",
                )
                .attr(
                    "xmlns:w16cid",
                    "http://schemas.microsoft.com/office/word/2016/wordml/cid",
                )
                .attr(
                    "xmlns:w16",
                    "http://schemas.microsoft.com/office/word/2018/wordml",
                )
                .attr(
                    "xmlns:w16se",
                    "http://schemas.microsoft.com/office/word/2015/wordml/symex",
                )
                .attr(
                    "xmlns:wpg",
                    "http://schemas.microsoft.com/office/word/2010/wordprocessingGroup",
                )
                .attr(
                    "xmlns:wpi",
                    "http://schemas.microsoft.com/office/word/2010/wordprocessingInk",
                )
                .attr(
                    "xmlns:wne",
                    "http://schemas.microsoft.com/office/word/2006/wordml",
                )
                .attr("xmlns:wps", "http://schemas.microsoft.com/office/word/2010"),
        )
    }
}
