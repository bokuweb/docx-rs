use super::XMLBuilder;
use super::XmlEvent;
use crate::types::*;

const EXPECT_MESSAGE: &str = "should write buf";

impl XMLBuilder {
    // i.e. <w:body... >
    opened_el!(open_body, "w:body");
    // i.e. <w:basedOn ... >
    only_str_val_el!(based_on, "w:basedOn");
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
    // i.e. <w:r ... >
    opened_el!(open_run, "w:r");
    opened_el!(open_run_property, "w:rPr");
    opened_el!(open_run_property_default, "w:rPrDefault");
    // i.e. <w:qFormat ... >
    closed_el!(q_format, "w:qFormat");
    // i.e. <w:p ... >
    // opened_el!(open_paragraph, "w:p");
    opened_el_with_attrs!(open_paragraph, "w:p");
    opened_el!(open_paragraph_property, "w:pPr");
    opened_el!(open_doc_defaults, "w:docDefaults");
    // i.e. <w:name ... >
    only_str_val_el!(name, "w:name");
    // i.e. <w:jc ... >
    only_str_val_el!(justification, "w:jc");
    // i.e. <w:pStyle ... >
    only_str_val_el!(paragraph_style, "w:pStyle");
    // i.e. <w:sz ... >
    only_usize_val_el!(sz, "w:sz");
    // i.e. <w:szCs ... >
    only_usize_val_el!(sz_cs, "w:szCs");

    closed_el!(b, "w:b");
    closed_el!(b_cs, "w:bCs");

    closed_el!(i, "w:i");
    closed_el!(i_cs, "w:iCs");
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
    only_str_val_el!(next, "w:next");

    // i.e. <w:color ... >
    only_str_val_el!(color, "w:color");

    // i.e. <w:highlight ... >
    only_str_val_el!(highlight, "w:highlight");

    // i.e. <w:ind ... >
    pub(crate) fn indent(mut self, left: usize, special_indent: Option<SpecialIndentType>) -> Self {
        let left = &format!("{}", left);
        let base = XmlEvent::start_element("w:ind").attr("w:left", left);
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
    opened_el!(open_table, "w:tbl");
    opened_el!(open_table_property, "w:tblPr");
    opened_el!(open_table_grid, "w:tblGrid");
    opened_el!(open_table_row, "w:tr");
    opened_el!(open_table_row_property, "w:trPr");
    opened_el!(open_table_cell, "w:tc");
    opened_el!(open_table_cell_property, "w:tcPr");
    opened_el!(open_table_cell_borders, "w:tcBorders");
    opened_el!(open_table_borders, "w:tblBorders");
    opened_el!(open_table_cell_margins, "w:tblCellMar");

    closed_w_with_type_el!(table_width, "w:tblW");
    closed_w_with_type_el!(table_indent, "w:tblInd");
    closed_w_with_type_el!(grid_column, "w:gridCol");
    closed_w_with_type_el!(table_cell_width, "w:tcW");

    only_usize_val_el!(grid_span, "w:gridSpan");
    only_str_val_el!(vertical_merge, "w:vMerge");

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

    closed_el!(shd, "w:shd", "w:fill", "w:val");

    closed_el!(tab, "w:tab");
    closed_el!(br, "w:br", "w:type");
    closed_el!(zoom, "w:zoom", "w:percent");
    only_usize_val_el!(default_tab_stop, "w:defaultTabStop");
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
