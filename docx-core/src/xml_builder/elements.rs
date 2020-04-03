use super::XMLBuilder;
use super::XmlEvent;
use crate::types::*;

const EXPECT_MESSAGE: &str = "should write buf";

impl XMLBuilder {
    // i.e. <w:body... >
    open!(open_body, "w:body");
    // i.e. <w:basedOn ... >
    closed_with_str!(based_on, "w:basedOn");
    // i.e. <w:t ... >
    pub(crate) fn text(mut self, text: &str, preserve_space: bool) -> Self {
        let space = if preserve_space {
            "preserve"
        } else {
            "default"
        };
        self.writer
            .write(XmlEvent::start_element("w:t").attr("xml:space", space))
            .expect(EXPECT_MESSAGE);
        self.writer.write(text).expect(EXPECT_MESSAGE);
        self.close()
    }
    // i.e. <w:delText ... >
    pub(crate) fn delete_text(mut self, text: &str, preserve_space: bool) -> Self {
        let space = if preserve_space {
            "preserve"
        } else {
            "default"
        };
        self.writer
            .write(XmlEvent::start_element("w:delText").attr("xml:space", space))
            .expect(EXPECT_MESSAGE);
        self.writer.write(text).expect(EXPECT_MESSAGE);
        self.close()
    }
    // i.e. <w:r ... >
    open!(open_run, "w:r");
    open!(open_run_property, "w:rPr");
    open!(open_run_property_default, "w:rPrDefault");
    // i.e. <w:qFormat ... >
    closed!(q_format, "w:qFormat");
    // i.e. <w:p ... >
    // open!(open_paragraph, "w:p");
    open_with_attrs!(open_paragraph, "w:p");
    open!(open_paragraph_property, "w:pPr");
    open!(open_doc_defaults, "w:docDefaults");
    // i.e. <w:name ... >
    closed_with_str!(name, "w:name");
    // i.e. <w:jc ... >
    closed_with_str!(justification, "w:jc");
    // i.e. <w:pStyle ... >
    closed_with_str!(paragraph_style, "w:pStyle");
    // i.e. <w:sz ... >
    closed_with_usize!(sz, "w:sz");
    // i.e. <w:szCs ... >
    closed_with_usize!(sz_cs, "w:szCs");

    closed!(b, "w:b");
    closed!(b_cs, "w:bCs");

    closed!(i, "w:i");
    closed!(i_cs, "w:iCs");
    // Build w:style element
    // i.e. <w:style ... >
    pub(crate) fn open_style(mut self, style_type: StyleType, id: &str) -> Self {
        self.writer
            .write(
                XmlEvent::start_element("w:style")
                    .attr("w:type", &style_type.to_string())
                    .attr("w:styleId", id),
            )
            .expect(EXPECT_MESSAGE);
        self
    }
    // i.e. <w:next ... >
    closed_with_str!(next, "w:next");

    // i.e. <w:color ... >
    closed_with_str!(color, "w:color");

    // i.e. <w:highlight ... >
    closed_with_str!(highlight, "w:highlight");

    // i.e. <w:u ... >
    closed_with_str!(underline, "w:u");

    // i.e. <w:ind ... >
    pub(crate) fn indent(
        mut self,
        start: Option<i32>,
        special_indent: Option<SpecialIndentType>,
        end: i32,
        start_chars: Option<i32>,
    ) -> Self {
        let start = &format!("{}", start.unwrap_or(0));
        let end = &format!("{}", end);
        let start_chars_value = format!("{}", start_chars.unwrap_or(0));
        let mut base = XmlEvent::start_element("w:ind")
            .attr("w:left", start)
            .attr("w:right", end);

        if start_chars.is_some() {
            base = base.attr("w:leftChars", &start_chars_value);
        }

        match special_indent {
            Some(SpecialIndentType::FirstLine(v)) => self
                .writer
                .write(base.attr("w:firstLine", &format!("{}", v)))
                .expect(EXPECT_MESSAGE),
            Some(SpecialIndentType::Hanging(v)) => self
                .writer
                .write(base.attr("w:hanging", &format!("{}", v)))
                .expect(EXPECT_MESSAGE),
            _ => self.writer.write(base).expect(EXPECT_MESSAGE),
        };
        self.close()
    }

    //
    // Table elements
    //
    open!(open_table, "w:tbl");
    open!(open_table_property, "w:tblPr");
    open!(open_table_grid, "w:tblGrid");
    open!(open_table_row, "w:tr");
    open!(open_table_row_property, "w:trPr");
    open!(open_table_cell, "w:tc");
    open!(open_table_cell_property, "w:tcPr");
    open!(open_table_cell_borders, "w:tcBorders");
    open!(open_table_borders, "w:tblBorders");
    open!(open_table_cell_margins, "w:tblCellMar");

    closed_w_with_type_el!(table_width, "w:tblW");
    closed_w_with_type_el!(table_indent, "w:tblInd");
    closed_w_with_type_el!(grid_column, "w:gridCol");
    closed_w_with_type_el!(table_cell_width, "w:tcW");

    closed_with_usize!(grid_span, "w:gridSpan");
    closed_with_str!(vertical_merge, "w:vMerge");
    closed_with_str!(vertical_align, "w:vAlign");

    closed_w_with_type_el!(margin_top, "w:top");
    closed_w_with_type_el!(margin_left, "w:left");
    closed_w_with_type_el!(margin_bottom, "w:bottom");
    closed_w_with_type_el!(margin_right, "w:right");

    closed_border_el!(border_top, "w:top");
    closed_border_el!(border_left, "w:left");
    closed_border_el!(border_bottom, "w:bottom");
    closed_border_el!(border_right, "w:right");
    closed_border_el!(border_inside_h, "w:insideH");
    closed_border_el!(border_inside_v, "w:insideV");

    closed!(shd, "w:shd", "w:fill", "w:val");

    closed!(tab, "w:tab");
    closed!(tab_with_pos, "w:tab", "w:val", "w:pos");

    closed!(br, "w:br", "w:type");
    closed!(zoom, "w:zoom", "w:percent");
    closed_with_usize!(default_tab_stop, "w:defaultTabStop");

    open!(open_font, "w:font", "w:name");
    closed_with_str!(pitch, "w:pitch");
    closed_with_str!(family, "w:family");
    closed_with_str!(charset, "w:charset");

    open!(open_section_property, "w:sectPr");
    closed_with_str!(type_tag, "w:type");
    closed!(page_size, "w:pgSz", "w:w", "w:h");
    closed!(
        page_margin,
        "w:pgMar",
        "w:top",
        "w:right",
        "w:bottom",
        "w:left",
        "w:header",
        "w:footer",
        "w:gutter"
    );
    closed!(columns, "w:cols", "w:space");
    closed!(document_grid, "w:docGrid", "w:type", "w:linePitch");

    open!(open_insert, "w:ins", "w:id", "w:author", "w:date");
    open!(open_delete, "w:del", "w:id", "w:author", "w:date");

    closed!(bookmark_start, "w:bookmarkStart", "w:id", "w:name");
    closed!(bookmark_end, "w:bookmarkEnd", "w:id");

    closed!(comment_range_start, "w:commentRangeStart", "w:id");
    closed!(comment_range_end, "w:commentRangeEnd", "w:id");
    closed!(comment_reference, "w:commentReference", "w:id");
    open!(
        open_comment,
        "w:comment",
        "w:id",
        "w:author",
        "w:date",
        "w:initials"
    );

    open!(open_abstract_num, "w:abstractNum", "w:abstractNumId");
    open!(open_level, "w:lvl", "w:ilvl");
    open!(open_tabs, "w:tabs");
    open!(open_num, "w:num", "w:numId");
    open!(open_numbering_property, "w:numPr");
    closed_with_usize!(indent_level, "w:ilvl");
    closed_with_usize!(num_id, "w:numId");
    closed_with_usize!(start, "w:start");
    closed_with_str!(number_format, "w:numFmt");
    closed_with_str!(level_text, "w:lvlText");
    closed_with_str!(level_justification, "w:lvlJc");
    closed_with_str!(abstract_num_id, "w:abstractNumId");
    closed!(vanish, "w:vanish");

    open!(open_drawing, "w:drawing");
    open!(open_anchor, "wp:anchor");
    open!(open_graphic, "a:graphic", "xmlns:a");
    open!(open_graphic_data, "a:graphicData", "uri");
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_sz() {
        let b = XMLBuilder::new();
        let r = b.sz(20).build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:sz w:val="20" />"#);
    }

    #[test]
    fn test_declaration() {
        let b = XMLBuilder::new();
        let r = b
            .open_style(StyleType::Paragraph, "Heading")
            .close()
            .build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<w:style w:type="paragraph" w:styleId="Heading" />"#
        );
    }

    #[test]
    fn test_next() {
        let b = XMLBuilder::new();
        let r = b.next("Normal").build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:next w:val="Normal" />"#);
    }

    #[test]
    fn test_name() {
        let b = XMLBuilder::new();
        let r = b.name("Heading").build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:name w:val="Heading" />"#);
    }

    #[test]
    fn test_color() {
        let b = XMLBuilder::new();
        let r = b.color("2E74B5").build();
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:color w:val="2E74B5" />"#);
    }

    #[test]
    fn test_based_on() {
        let b = XMLBuilder::new();
        let r = b.based_on("Normal").build();
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<w:basedOn w:val="Normal" />"#
        );
    }
}
