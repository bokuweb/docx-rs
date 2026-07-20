use super::XMLBuilder;
use super::XmlEvent;
use crate::types::*;
use crate::FrameProperty;
use crate::TablePositionProperty;

use crate::xml::writer::Result;
use std::io::Write;

impl<W: Write> XMLBuilder<W> {
    // i.e. <w:body... >
    open!(open_body, "w:body");
    // i.e. <w:basedOn ... >
    closed_with_str!(based_on, "w:basedOn");
    // i.e. <w:t ... >
    pub(crate) fn text(self, text: &str, preserve_space: bool) -> Result<Self> {
        let space = if preserve_space {
            "preserve"
        } else {
            "default"
        };
        self.write(XmlEvent::start_element("w:t").attr("xml:space", space))?
            .write(text)?
            .close()
    }

    pub(crate) fn snap_to_grid(self, v: bool) -> Result<Self> {
        self.write(XmlEvent::start_element("w:snapToGrid").attr_display("w:val", v))?
            .close()
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn run_fonts(
        self,
        ascii: Option<&String>,
        hi_ansi: Option<&String>,
        cs: Option<&String>,
        east_asia: Option<&String>,
        ascii_theme: Option<&String>,
        hi_ansi_theme: Option<&String>,
        cs_theme: Option<&String>,
        east_asia_theme: Option<&String>,
        hint: Option<&String>,
    ) -> Result<Self> {
        let mut w = XmlEvent::start_element("w:rFonts");
        if let Some(ascii) = ascii {
            w = w.attr("w:ascii", ascii);
        }
        if let Some(hi_ansi) = hi_ansi {
            w = w.attr("w:hAnsi", hi_ansi);
        }
        if let Some(cs) = cs {
            w = w.attr("w:cs", cs);
        }
        if let Some(east_asia) = east_asia {
            w = w.attr("w:eastAsia", east_asia);
        }
        if let Some(ascii_theme) = ascii_theme {
            w = w.attr("w:asciiTheme", ascii_theme);
        }
        if let Some(hi_ansi_theme) = hi_ansi_theme {
            w = w.attr("w:hAnsiTheme", hi_ansi_theme);
        }
        if let Some(cs_theme) = cs_theme {
            w = w.attr("w:cstheme", cs_theme);
        }
        if let Some(east_asia_theme) = east_asia_theme {
            w = w.attr("w:eastAsiaTheme", east_asia_theme);
        }
        if let Some(hint) = hint {
            w = w.attr("w:hint", hint);
        }
        self.write(w)?.close()
    }

    // i.e. <w:delText ... >
    pub(crate) fn delete_text(self, text: &str, preserve_space: bool) -> Result<Self> {
        let space = if preserve_space {
            "preserve"
        } else {
            "default"
        };
        self.write(XmlEvent::start_element("w:delText").attr("xml:space", space))?
            .write(text)?
            .close()
    }

    pub(crate) fn data_binding(
        self,
        xpath: Option<&String>,
        prefix_mappings: Option<&String>,
        store_item_id: Option<&String>,
    ) -> Result<Self> {
        let mut e = XmlEvent::start_element("w:dataBinding");
        if let Some(xpath) = xpath {
            e = e.attr("w:xpath", xpath);
        }
        if let Some(prefix_mappings) = prefix_mappings {
            e = e.attr("w:prefixMappings", prefix_mappings);
        }
        if let Some(store_item_id) = store_item_id {
            e = e.attr("w:storeItemID", store_item_id);
        }
        self.write(e)?.close()
    }

    pub(crate) fn open_hyperlink(
        self,
        rid: Option<&String>,
        anchor: Option<&String>,
        history: Option<usize>,
    ) -> Result<Self> {
        let mut e = XmlEvent::start_element("w:hyperlink");
        let history = history.unwrap_or(1);
        if let Some(rid) = rid {
            e = e.attr("r:id", rid);
        }
        if let Some(anchor) = anchor {
            e = e.attr("w:anchor", anchor);
        }
        e = e.attr_display("w:history", history);
        self.write(e)
    }

    // i.e. <w:r ... >
    open!(open_run, "w:r");
    open!(open_run_property, "w:rPr");
    open!(open_paragraph_borders, "w:pBdr");
    open!(open_run_property_default, "w:rPrDefault");
    open!(open_paragraph_property_default, "w:pPrDefault");
    // i.e. <w:qFormat ... >
    closed!(q_format, "w:qFormat");
    // i.e. <w:uiPriority ... >
    closed_with_usize!(ui_priority, "w:uiPriority");
    // i.e. <w:semiHidden ... >
    closed!(semi_hidden, "w:semiHidden");
    // i.e. <w:unhideWhenUsed ... >
    closed!(unhide_when_used, "w:unhideWhenUsed");
    // i.e. <w:p ... >
    // open!(open_paragraph, "w:p");
    open!(open_paragraph, "w:p", "w14:paraId");
    open!(open_paragraph_property, "w:pPr");
    open!(open_doc_defaults, "w:docDefaults");

    open!(open_structured_tag, "w:sdt");
    open!(open_structured_tag_content, "w:sdtContent");
    open!(open_structured_tag_property, "w:sdtPr");
    closed_with_str!(alias, "w:alias");

    closed_paragraph_border_el!(paragraph_border_top, "w:top");
    closed_paragraph_border_el!(paragraph_border_left, "w:left");
    closed_paragraph_border_el!(paragraph_border_bottom, "w:bottom");
    closed_paragraph_border_el!(paragraph_border_right, "w:right");
    closed_paragraph_border_el!(paragraph_border_between, "w:between");
    closed_paragraph_border_el!(paragraph_border_bar, "w:bar");

    // i.e. <w:outlineLvl ...>
    closed_with_usize!(outline_lvl, "w:outlineLvl");
    // i.e. <w:name ... >
    closed_with_str!(name, "w:name");
    // i.e. <w:jc ... >
    closed_with_str!(justification, "w:jc");
    // i.e. <w:vertAlign ... >
    closed_with_str!(vert_align, "w:vertAlign");
    // i.e. <w:pStyle ... >
    closed_with_str!(paragraph_style, "w:pStyle");
    // i.e. <w:rStyle ... >
    closed_with_str!(run_style, "w:rStyle");
    // i.e. <w:sz ... >
    closed_with_usize!(sz, "w:sz");
    // i.e. <w:szCs ... >
    closed_with_usize!(sz_cs, "w:szCs");
    closed_with_isize!(adjust_right_ind, "w:adjustRightInd");
    closed_with_str!(text_alignment, "w:textAlignment");

    closed!(field_character, "w:fldChar", "w:fldCharType", "w:dirty");

    open!(open_instr_text, "w:instrText");
    open!(open_delete_instr_text, "w:delInstrText");

    closed!(text_direction, "w:textDirection", "w:val");

    closed!(b, "w:b");
    closed!(b_cs, "w:bCs");

    pub(crate) fn disable_bold(self) -> Result<Self> {
        let f = "false";
        self.write(XmlEvent::start_element("w:b").attr("w:val", f))?
            .close()
    }

    closed_with_str!(caps, "w:caps");

    closed!(i, "w:i");
    closed!(i_cs, "w:iCs");

    pub(crate) fn disable_italic(self) -> Result<Self> {
        let f = "false";
        self.write(XmlEvent::start_element("w:i").attr("w:val", f))?
            .close()
    }

    closed!(strike, "w:strike");

    pub(crate) fn disable_strike(self) -> Result<Self> {
        let f = "false";
        self.write(XmlEvent::start_element("w:strike").attr("w:val", f))?
            .close()
    }

    closed!(dstrike, "w:dstrike");

    pub(crate) fn disable_dstrike(self) -> Result<Self> {
        let f = "false";
        self.write(XmlEvent::start_element("w:dstrike").attr("w:val", f))?
            .close()
    }

    // Build w:style element
    // i.e. <w:style ... >
    pub(crate) fn open_style(self, style_type: StyleType, id: &str) -> Result<Self> {
        self.write(
            XmlEvent::start_element("w:style")
                .attr_display("w:type", style_type)
                .attr("w:styleId", id),
        )
    }
    // i.e. <w:next ... >
    closed_with_str!(next, "w:next");

    closed_with_str!(link, "w:link");

    // i.e. <w:color ... >
    closed_with_str!(color, "w:color");

    // i.e. <w:color w:val=".." w:themeColor=".." w:themeShade=".." w:themeTint=".." />
    //
    // Theme attributes are optional: when all three are `None` the output is
    // byte-for-byte identical to `color()`, so existing callers are unaffected.
    // Attributes are appended only when present, matching how Word emits its
    // own built-in styles (the `w:val` hex stays as a non-theme fallback).
    pub(crate) fn color_with_theme(
        self,
        val: &str,
        theme_color: Option<&str>,
        theme_shade: Option<&str>,
        theme_tint: Option<&str>,
    ) -> Result<Self> {
        let mut el = XmlEvent::start_element("w:color").attr("w:val", val);
        if let Some(tc) = theme_color {
            el = el.attr("w:themeColor", tc);
        }
        if let Some(ts) = theme_shade {
            el = el.attr("w:themeShade", ts);
        }
        if let Some(tt) = theme_tint {
            el = el.attr("w:themeTint", tt);
        }
        self.write(el)?.close()
    }

    // i.e. <w:highlight ... >
    closed_with_str!(highlight, "w:highlight");

    // i.e. <w:u ... >
    closed_with_str!(underline, "w:u");

    closed_with_str!(suffix, "w:suff");

    // i.e. <w:ind ... >
    pub(crate) fn indent(
        self,
        start: Option<i32>,
        special_indent: Option<SpecialIndentType>,
        end: i32,
        start_chars: Option<i32>,
    ) -> Result<Self> {
        let mut base = XmlEvent::start_element("w:ind")
            .attr_display("w:left", start.unwrap_or(0))
            .attr_display("w:right", end);

        if let Some(start_chars) = start_chars {
            base = base.attr_display("w:leftChars", start_chars);
        }

        match special_indent {
            Some(SpecialIndentType::FirstLine(v)) => {
                self.write(base.attr_display("w:firstLine", v))
            }
            Some(SpecialIndentType::Hanging(v)) => self.write(base.attr_display("w:hanging", v)),
            None => self.write(base),
        }?
        .close()
    }

    // i.e. <w:spacing ... >
    pub(crate) fn spacing(self, s: i32) -> Result<Self> {
        self.write(XmlEvent::start_element("w:spacing").attr_display("w:val", s))?
            .close()
    }

    // i.e. <w:w ... >
    pub(crate) fn w(self, s: i32) -> Result<Self> {
        self.write(XmlEvent::start_element("w:w").attr_display("w:val", s))?
            .close()
    }

    // i.e. <w:fitText ... >
    pub(crate) fn fit_text(self, val: usize, id: Option<u32>) -> Result<Self> {
        let mut fit_text = XmlEvent::start_element("w:fitText").attr_display("w:val", val);
        if let Some(id) = id {
            fit_text = fit_text.attr_display("w:id", id);
        }
        self.write(fit_text)?.close()
    }

    // i.e. <w:spacing ... >
    pub(crate) fn line_spacing(
        self,
        before: Option<u32>,
        after: Option<u32>,
        line: Option<i32>,
        before_lines: Option<u32>,
        after_lines: Option<u32>,
        spacing: Option<LineSpacingType>,
    ) -> Result<Self> {
        let mut xml_event = XmlEvent::start_element("w:spacing");
        if let Some(before) = before {
            xml_event = xml_event.attr_display("w:before", before)
        }
        if let Some(after) = after {
            xml_event = xml_event.attr_display("w:after", after)
        }
        if let Some(before_lines) = before_lines {
            xml_event = xml_event.attr_display("w:beforeLines", before_lines)
        }
        if let Some(after_lines) = after_lines {
            xml_event = xml_event.attr_display("w:afterLines", after_lines)
        }
        if let Some(line) = line {
            xml_event = xml_event.attr_display("w:line", line)
        }
        if let Some(spacing_type) = spacing {
            match spacing_type {
                LineSpacingType::Auto => {
                    xml_event = xml_event.attr("w:lineRule", "auto");
                }
                LineSpacingType::AtLeast => {
                    xml_event = xml_event.attr("w:lineRule", "atLeast");
                }
                LineSpacingType::Exact => {
                    xml_event = xml_event.attr("w:lineRule", "exact");
                }
            }
        }
        self.write(xml_event)?.close()
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
    open!(open_cell_margins, "w:tcMar");

    closed!(table_layout, "w:tblLayout", "w:type");
    closed_with_str!(table_style, "w:tblStyle");
    closed_w_with_type_el!(table_width, "w:tblW");
    closed_w_with_type_el!(table_indent, "w:tblInd");
    closed_w_with_type_el!(grid_column, "w:gridCol");
    closed_w_with_type_el!(table_cell_width, "w:tcW");

    closed!(table_row_height, "w:trHeight", "w:val", "w:hRule");

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
    closed_border_el!(border_tl2br, "w:tl2br");
    closed_border_el!(border_tr2bl, "w:tr2bl");

    closed_border_el!(text_border, "w:bdr");

    closed!(shd, "w:shd", "w:val", "w:color", "w:fill");

    closed!(tab_with_pos, "w:tab", "w:val", "w:pos");

    closed!(br, "w:br", "w:type");
    closed!(cr, "w:cr");
    closed!(sym, "w:sym", "w:font", "w:char");
    closed!(zoom, "w:zoom", "w:percent");
    closed_with_usize!(default_tab_stop, "w:defaultTabStop");

    open!(open_font, "w:font", "w:name");
    closed_with_str!(pitch, "w:pitch");
    closed_with_str!(family, "w:family");
    closed_with_str!(charset, "w:charset");

    open!(open_section_property, "w:sectPr");
    closed!(header_reference, "w:headerReference", "w:type", "r:id");
    closed!(footer_reference, "w:footerReference", "w:type", "r:id");

    closed_with_str!(type_tag, "w:type");
    closed!(title_pg, "w:titlePg");
    closed!(even_and_odd_headers, "w:evenAndOddHeaders");
    closed!(page_size, "w:pgSz", "w:w", "w:h");
    closed!(page_size_with_orient, "w:pgSz", "w:w", "w:h", "w:orient");
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
    closed!(columns, "w:cols", "w:space", "w:num");
    // closed!(text_direction, "w:val");
    closed!(document_grid, "w:docGrid", "w:type", "w:linePitch");

    open!(open_insert, "w:ins", "w:id", "w:author", "w:date");
    open!(open_delete, "w:del", "w:id", "w:author", "w:date");
    open!(open_move_from, "w:moveFrom", "w:id", "w:author", "w:date");
    open!(open_move_to, "w:moveTo", "w:id", "w:author", "w:date");
    open!(
        open_paragraph_property_change,
        "w:pPrChange",
        "w:id",
        "w:author",
        "w:date"
    );
    // cantSplit for table row
    closed!(cant_split, "w:cantSplit");

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

    closed_with_str!(multi_level_type, "w:multiLevelType");

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
    closed_with_str!(level_restart, "w:lvlRestart");
    closed_with_str!(level_justification, "w:lvlJc");
    closed_with_str!(abstract_num_id, "w:abstractNumId");
    closed!(vanish, "w:vanish");
    closed!(spec_vanish, "w:specVanish");
    closed!(is_lgl, "w:isLgl");

    open!(open_drawing, "w:drawing");
    open!(open_anchor, "wp:anchor");
    open!(open_graphic, "a:graphic", "xmlns:a");
    open!(open_graphic_data, "a:graphicData", "uri");

    // shape
    open!(open_wp_shape, "wps:wsp");
    open!(open_wp_text_box, "wps:txbx");
    open!(open_text_box_content, "w:txbxContent");

    // compat
    open!(open_compat, "w:compat");
    closed!(space_for_ul, "w:spaceForUL");
    closed!(
        balance_single_byte_double_byte_width,
        "w:balanceSingleByteDoubleByteWidth"
    );
    closed!(do_not_leave_backslash_alone, "w:doNotLeaveBackslashAlone");
    closed!(ul_trail_space, "w:ulTrailSpace");
    closed!(do_not_expand_shift_return, "w:doNotExpandShiftReturn");
    closed!(adjust_line_height_table, "w:adjustLineHeightInTable");
    closed!(
        character_spacing_control,
        "w:characterSpacingControl",
        "w:val"
    );
    closed!(use_fe_layout, "w:useFELayout");
    closed!(
        compat_setting,
        "w:compatSetting",
        "w:name",
        "w:uri",
        "w:val"
    );

    closed!(keep_next, "w:keepNext");
    closed!(keep_lines, "w:keepLines");
    closed!(page_break_before, "w:pageBreakBefore");
    closed!(widow_control, "w:widowControl", "w:val");
    closed!(bidi, "w:bidi");
    /*
    <w:lvlOverride w:ilvl="0">
      <w:startOverride w:val="1"/>
    </w:lvlOverride>
    */
    open!(open_level_override, "w:lvlOverride", "w:ilvl");
    closed_with_str!(start_override, "w:startOverride");

    closed!(doc_id, "w15:docId", "w15:val");

    open!(open_doc_vars, "w:docVars");
    closed!(doc_var, "w:docVar", "w:name", "w:val");

    // webextension
    open!(open_webextension, "we:webextension", "xmlns:we", "id");
    closed!(
        webextension_reference,
        "we:reference",
        "id",
        "version",
        "store",
        "storeType"
    );
    closed!(webextension_alternate_references, "we:alternateReferences");
    open!(open_webextension_properties, "we:properties");
    closed!(webextension_property, "we:property", "name", "value");
    closed!(webextension_bindings, "we:bindings");
    closed!(webextension_snapshot, "we:snapshot", "xmlns:r");

    // taskpanes
    open!(open_taskpanes, "wetp:taskpanes", "xmlns:wetp");
    open!(
        open_taskpane,
        "wetp:taskpane",
        "dockstate",
        "visibility",
        "width",
        "row"
    );
    closed!(webextensionref, "wetp:webextensionref", "xmlns:r", "r:id");

    // customXML
    open!(
        open_data_store_item,
        "ds:datastoreItem",
        "xmlns:ds",
        "ds:itemID"
    );
    open!(open_data_store_schema_refs, "ds:schemaRefs");

    // CommentExtended
    // w15:commentEx w15:paraId="00000001" w15:paraIdParent="57D1BD7C" w15:done="0"
    pub(crate) fn comment_extended(
        self,
        paragraph_id: &str,
        done: bool,
        parent_paragraph_id: &Option<String>,
    ) -> Result<Self> {
        let el = match parent_paragraph_id {
            Some(parent_paragraph_id) => XmlEvent::start_element("w15:commentEx")
                .attr("w15:paraId", paragraph_id)
                .attr("w15:paraIdParent", parent_paragraph_id)
                .attr_display("w15:done", done as usize),
            None => XmlEvent::start_element("w15:commentEx")
                .attr("w15:paraId", paragraph_id)
                .attr_display("w15:done", done as usize),
        };
        self.write(el)?.close()
    }

    // docGrid
    pub(crate) fn doc_grid(
        self,
        t: &DocGridType,
        line_pitch: Option<usize>,
        char_space: Option<isize>,
    ) -> Result<Self> {
        let mut w = XmlEvent::start_element("w:docGrid").attr_display("w:type", t);
        if let Some(line_pitch) = line_pitch {
            w = w.attr_display("w:linePitch", line_pitch);
        }
        if let Some(char_space) = char_space {
            w = w.attr_display("w:charSpace", char_space);
        }
        self.write(w)?.close()
    }

    pub(crate) fn frame_property(self, prop: &FrameProperty) -> Result<Self> {
        let mut w = XmlEvent::start_element("w:framePr");
        let wrap: String = prop.wrap.iter().cloned().collect();
        if prop.wrap.is_some() {
            w = w.attr("w:wrap", &wrap);
        }
        let h_rule: String = prop.h_rule.iter().cloned().collect();
        if prop.h_rule.is_some() {
            w = w.attr("w:hRule", &h_rule);
        }
        let h_anchor: String = prop.h_anchor.iter().cloned().collect();
        if prop.h_anchor.is_some() {
            w = w.attr("w:hAnchor", &h_anchor);
        }
        let v_anchor: String = prop.v_anchor.iter().cloned().collect();
        if prop.v_anchor.is_some() {
            w = w.attr("w:vAnchor", &v_anchor);
        }
        let x_align: String = prop.x_align.iter().cloned().collect();
        if prop.x_align.is_some() {
            w = w.attr("w:xAlign", &x_align);
        }
        let y_align: String = prop.y_align.iter().cloned().collect();
        if prop.y_align.is_some() {
            w = w.attr("w:yAlign", &y_align);
        }
        if let Some(x) = prop.x {
            w = w.attr_display("w:x", x);
        }
        if let Some(y) = prop.y {
            w = w.attr_display("w:y", y);
        }
        if let Some(h_space) = prop.h_space {
            w = w.attr_display("w:h_space", h_space);
        }
        if let Some(v_space) = prop.v_space {
            w = w.attr_display("w:v_space", v_space);
        }
        if let Some(width) = prop.w {
            w = w.attr_display("w:w", width);
        }
        if let Some(height) = prop.h {
            w = w.attr_display("w:h", height);
        }
        self.write(w)?.close()
    }

    pub(crate) fn table_position_property(self, prop: &TablePositionProperty) -> Result<Self> {
        let mut w = XmlEvent::start_element("w:tblpPr");

        if let Some(left_from_text) = prop.left_from_text {
            w = w.attr_display("w:leftFromText", left_from_text);
        }

        if let Some(right_from_text) = prop.right_from_text {
            w = w.attr_display("w:rightFromText", right_from_text);
        }

        let v: String = prop.vertical_anchor.iter().cloned().collect();
        if prop.vertical_anchor.is_some() {
            w = w.attr("w:vertAnchor", &v);
        }

        let v: String = prop.horizontal_anchor.iter().cloned().collect();
        if prop.horizontal_anchor.is_some() {
            w = w.attr("w:horzAnchor", &v);
        }

        let v: String = prop.position_x_alignment.iter().cloned().collect();
        if prop.position_x_alignment.is_some() {
            w = w.attr("w:tblpXSpec", &v);
        }

        let v: String = prop.position_y_alignment.iter().cloned().collect();
        if prop.position_y_alignment.is_some() {
            w = w.attr("w:tblpYSpec", &v);
        }

        if let Some(position_x) = prop.position_x {
            w = w.attr_display("w:tblpX", position_x);
        }

        if let Some(position_y) = prop.position_y {
            w = w.attr_display("w:tblpY", position_y);
        }

        self.write(w)?.close()
    }

    pub(crate) fn page_num_type(
        self,
        start: Option<u32>,
        chap_style: Option<String>,
    ) -> Result<Self> {
        let mut w = XmlEvent::start_element("w:pgNumType");
        if let Some(start) = start {
            w = w.attr_display("w:start", start);
        }
        if let Some(chap_style) = chap_style.as_deref() {
            w = w.attr("w:chapStyle", chap_style);
        }
        self.write(w)?.close()
    }

    pub(crate) fn tab(
        self,
        v: Option<TabValueType>,
        leader: Option<TabLeaderType>,
        pos: Option<usize>,
    ) -> Result<Self> {
        let mut t = XmlEvent::start_element("w:tab");
        if let Some(v) = v {
            t = t.attr_display("w:val", v);
        }

        if let Some(leader) = leader {
            t = t.attr_display("w:leader", leader);
        }

        if let Some(pos) = pos {
            t = t.attr_display("w:pos", pos);
        }
        self.write(t)?.close()
    }

    pub(crate) fn ptab(
        self,
        alignment: PositionalTabAlignmentType,
        relative_to: PositionalTabRelativeTo,
        leader: TabLeaderType,
    ) -> Result<Self> {
        let t = XmlEvent::start_element("w:ptab")
            .attr_display("w:alignment", alignment)
            .attr_display("w:relativeTo", relative_to)
            .attr_display("w:leader", leader);

        self.write(t)?.close()
    }

    // FootnoteReference
    // w:footnoteReference w:id="1"
    pub(crate) fn footnote_reference(self, id: usize) -> Result<Self> {
        self.write(XmlEvent::start_element("w:footnoteReference").attr_display("w:id", id))?
            .close()
    }

    // Footnotes
    open!(open_footnote, "w:footnote", "w:id");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_sz() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b.sz(20)?.into_inner()?.into_inner()?;
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:sz w:val="20" />"#);
        Ok(())
    }

    #[test]
    fn test_declaration() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b
            .open_style(StyleType::Paragraph, "Heading")?
            .close()?
            .into_inner()?
            .into_inner()?;
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<w:style w:type="paragraph" w:styleId="Heading" />"#
        );
        Ok(())
    }

    #[test]
    fn test_next() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b.next("Normal")?.into_inner()?.into_inner()?;
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:next w:val="Normal" />"#);
        Ok(())
    }

    #[test]
    fn test_name() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b.name("Heading")?.into_inner()?.into_inner()?;
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:name w:val="Heading" />"#);
        Ok(())
    }

    #[test]
    fn test_color() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b.color("2E74B5")?.into_inner()?.into_inner()?;
        assert_eq!(str::from_utf8(&r).unwrap(), r#"<w:color w:val="2E74B5" />"#);
        Ok(())
    }

    #[test]
    fn test_based_on() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b.based_on("Normal")?.into_inner()?.into_inner()?;
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<w:basedOn w:val="Normal" />"#
        );
        Ok(())
    }

    #[test]
    fn test_ptab() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b
            .ptab(
                PositionalTabAlignmentType::Left,
                PositionalTabRelativeTo::Indent,
                TabLeaderType::None,
            )?
            .into_inner()?
            .into_inner()?;
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<w:ptab w:alignment="left" w:relativeTo="indent" w:leader="none" />"#
        );
        Ok(())
    }

    #[test]
    fn test_footnote_reference() -> Result<()> {
        let b = XMLBuilder::new(Vec::new());
        let r = b.footnote_reference(1)?.into_inner()?.into_inner()?;
        assert_eq!(
            str::from_utf8(&r).unwrap(),
            r#"<w:footnoteReference w:id="1" />"#
        );
        Ok(())
    }
}
