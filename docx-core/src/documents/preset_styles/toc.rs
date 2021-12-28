use serde::Serialize;

use super::*;
use crate::documents::*;
use crate::types::*;
use crate::xml_builder::*;

/*
<w:style w:type="paragraph" w:styleId="11">
    <w:name w:val="toc 1"/>
    <w:basedOn w:val="a"/>
    <w:next w:val="a"/>
    <w:autoRedefine/>
    <w:uiPriority w:val="39"/>
    <w:unhideWhenUsed/>
    <w:rsid w:val="0048082D"/>
</w:style>
<w:style w:type="paragraph" w:styleId="20">
    <w:name w:val="toc 2"/>
    <w:basedOn w:val="a"/>
    <w:next w:val="a"/>
    <w:autoRedefine/>
    <w:uiPriority w:val="39"/>
    <w:unhideWhenUsed/>
    <w:rsid w:val="0048082D"/>
    <w:pPr>
        <w:ind w:leftChars="100" w:left="200"/>
    </w:pPr>
</w:style>
 */
pub fn toc1() -> Style {
    Style::new("ToC1", StyleType::Paragraph)
        .name("toc 1")
        .align(AlignmentType::Both)
}

pub fn toc2() -> Style {
    Style::new("ToC2", StyleType::Paragraph)
        .name("toc 2")
        .align(AlignmentType::Both)
        .indent(Some(200), None, None, Some(100))
}
