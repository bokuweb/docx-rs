use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::EventReader;

use crate::reader::ReaderError;

#[derive(PartialEq, Debug)]
pub enum XMLElement {
    Body,
    Paragraph,
    Run,
    RunProperty,
    Color,
    Underline,
    Size,
    SizeCs,
    Vanish,
    Italic,
    ItalicCs,
    Text,
    Highlight,
    Bold,
    BoldCs,
    Break,
    Tab,
    ParagraphStyle,
    Indent,
    Name,
    Alignment,
    NumberingProperty,
    IndentLevel,
    NumberingId,
    Justification,
    Insert,
    Delete,
    DeleteText,
    BookmarkStart,
    BookmarkEnd,
    CommentRangeStart,
    CommentRangeEnd,
    VAlign,
    Table,
    TableProperty,
    TableRow,
    TableCell,
    TableCellProperty,
    TableCellWidth,
    TableCellBorders,
    TableVMerge,
    TableGridSpan,
    TableWidth,
    TableIndent,
    TableBorders,
    Top,
    Left,
    Bottom,
    InsideH,
    TableCellMargin,
    TableGrid,
    GridCol,
    Style,
    BasedOn,
    Next,
    VertAlign,
    Spacing,
    Styles,
    Relationship,
    Relationships,
    AbstractNumbering,
    AbstractNumberingId,
    Level,
    Numbering,
    Num,
    Start,
    NumberFormat,
    LevelText,
    LevelJustification,
    StyleLink,
    NumStyleLink,
    Drawing,
    TxbxContent,
    Pict,
    Unsupported,
}

#[derive(PartialEq, Debug)]
pub enum McXMLElement {
    AlternateContent,
    Choice,
    Fallback,
    Unsupported,
}

#[derive(PartialEq, Debug)]
pub enum WpXMLElement {
    Anchor,
    SimplePos,
    PositionH,
    PosOffset,
    PositionV,
    Extent,
    EffectExtent,
    WrapNone,
    DocProperty,
    Unsupported,
}

#[derive(PartialEq, Debug)]
pub enum AXMLElement {
    Graphic,
    GraphicData,
    Xfrm,
    Off,
    Ext,
    PrstGeom,
    SolidFill,
    Ln,
    Unsupported,
}

#[derive(PartialEq, Debug)]
pub enum WpsXMLElement {
    Wsp,
    CNvSpProperty,
    SpProperty,
    Style,
    Txbx,
    BodyPr,
    Unsupported,
}
#[derive(PartialEq, Debug)]
pub enum VXMLElement {
    Rect,
    Stroke,
    Fill,
    TextBox,
    Unsupported,
}

impl FromStr for XMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "body" => Ok(XMLElement::Body),
            "p" => Ok(XMLElement::Paragraph),
            "r" => Ok(XMLElement::Run),
            "rPr" => Ok(XMLElement::RunProperty),
            "color" => Ok(XMLElement::Color),
            "t" => Ok(XMLElement::Text),
            "sz" => Ok(XMLElement::Size),
            "szCs" => Ok(XMLElement::SizeCs),
            "u" => Ok(XMLElement::Underline),
            "pStyle" => Ok(XMLElement::ParagraphStyle),
            "highlight" => Ok(XMLElement::Highlight),
            "b" => Ok(XMLElement::Bold),
            "bCs" => Ok(XMLElement::BoldCs),
            "i" => Ok(XMLElement::Italic),
            "iCs" => Ok(XMLElement::ItalicCs),
            "vanish" => Ok(XMLElement::Vanish),
            "italic" => Ok(XMLElement::Italic),
            "name" => Ok(XMLElement::Name),
            "tab" => Ok(XMLElement::Tab),
            "br" => Ok(XMLElement::Break),
            "ind" => Ok(XMLElement::Indent),
            "numPr" => Ok(XMLElement::NumberingProperty),
            "ilvl" => Ok(XMLElement::IndentLevel),
            "numId" => Ok(XMLElement::NumberingId),
            "jc" => Ok(XMLElement::Justification),
            "ins" => Ok(XMLElement::Insert),
            "del" => Ok(XMLElement::Delete),
            "delText" => Ok(XMLElement::DeleteText),
            "bookmarkStart" => Ok(XMLElement::BookmarkStart),
            "bookmarkEnd" => Ok(XMLElement::BookmarkEnd),
            "commentRangeStart" => Ok(XMLElement::CommentRangeStart),
            "commentRangeEnd" => Ok(XMLElement::CommentRangeEnd),
            "tbl" => Ok(XMLElement::Table),
            "tblPr" => Ok(XMLElement::TableProperty),
            "tr" => Ok(XMLElement::TableRow),
            "tc" => Ok(XMLElement::TableCell),
            "tcPr" => Ok(XMLElement::TableCellProperty),
            "tcW" => Ok(XMLElement::TableCellWidth),
            "tcBorders" => Ok(XMLElement::TableCellBorders),
            "vMerge" => Ok(XMLElement::TableVMerge),
            "gridSpan" => Ok(XMLElement::TableGridSpan),
            "tblW" => Ok(XMLElement::TableWidth),
            "tblInd" => Ok(XMLElement::TableIndent),
            "tblBorders" => Ok(XMLElement::TableBorders),
            "top" => Ok(XMLElement::Top),
            "left" => Ok(XMLElement::Left),
            "bottom" => Ok(XMLElement::Bottom),
            "insideH" => Ok(XMLElement::InsideH),
            "tblCellMar" => Ok(XMLElement::TableCellMargin),
            "tblGrid" => Ok(XMLElement::TableGrid),
            "gridCol" => Ok(XMLElement::GridCol),
            "style" => Ok(XMLElement::Style),
            "basedOn" => Ok(XMLElement::BasedOn),
            "next" => Ok(XMLElement::Next),
            "vertAlign" => Ok(XMLElement::VertAlign),
            "spacing" => Ok(XMLElement::Spacing),
            "styles" => Ok(XMLElement::Styles),
            "Relationships" => Ok(XMLElement::Relationships),
            "Relationship" => Ok(XMLElement::Relationship),
            "abstractNum" => Ok(XMLElement::AbstractNumbering),
            "abstractNumId" => Ok(XMLElement::AbstractNumberingId),
            "lvl" => Ok(XMLElement::Level),
            "numbering" => Ok(XMLElement::Numbering),
            "num" => Ok(XMLElement::Num),
            "start" => Ok(XMLElement::Start),
            "numFmt" => Ok(XMLElement::NumberFormat),
            "lvlText" => Ok(XMLElement::LevelText),
            "lvlJc" => Ok(XMLElement::LevelJustification),
            "numStyleLink" => Ok(XMLElement::NumStyleLink),
            "styleLink" => Ok(XMLElement::StyleLink),
            "vAlign" => Ok(XMLElement::VAlign),
            "drawing" => Ok(XMLElement::Drawing),
            "txbxContent" => Ok(XMLElement::TxbxContent),
            "pict" => Ok(XMLElement::Pict),
            _ => Ok(XMLElement::Unsupported),
        }
    }
}

impl FromStr for McXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alternateContent" => Ok(McXMLElement::AlternateContent),
            "choice" => Ok(McXMLElement::Choice),
            "fallback" => Ok(McXMLElement::Fallback),
            _ => Ok(McXMLElement::Unsupported),
        }
    }
}

impl FromStr for WpXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "anchor" => Ok(WpXMLElement::Anchor),
            "simplePos" => Ok(WpXMLElement::SimplePos),
            "positionH" => Ok(WpXMLElement::PositionH),
            "posOffset" => Ok(WpXMLElement::PosOffset),
            "positionV" => Ok(WpXMLElement::PositionV),
            "extent" => Ok(WpXMLElement::Extent),
            "effectExtent" => Ok(WpXMLElement::EffectExtent),
            "wrapNone" => Ok(WpXMLElement::WrapNone),
            "docPr" => Ok(WpXMLElement::DocProperty),
            _ => Ok(WpXMLElement::Unsupported),
        }
    }
}

impl FromStr for AXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "graphic" => Ok(AXMLElement::Graphic),
            "graphicData" => Ok(AXMLElement::GraphicData),
            "xfrm" => Ok(AXMLElement::Xfrm),
            "off" => Ok(AXMLElement::Off),
            "ext" => Ok(AXMLElement::Ext),
            "prstGeom" => Ok(AXMLElement::PrstGeom),
            "solidFill" => Ok(AXMLElement::SolidFill),
            "ln" => Ok(AXMLElement::Ln),
            _ => Ok(AXMLElement::Unsupported),
        }
    }
}

impl FromStr for WpsXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wsp" => Ok(WpsXMLElement::Wsp),
            "cNvSpPr" => Ok(WpsXMLElement::CNvSpProperty),
            "spPr" => Ok(WpsXMLElement::SpProperty),
            "style" => Ok(WpsXMLElement::Style),
            "txbx" => Ok(WpsXMLElement::Txbx),
            "bodyPr" => Ok(WpsXMLElement::BodyPr),
            _ => Ok(WpsXMLElement::Unsupported),
        }
    }
}

impl FromStr for VXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rect" => Ok(VXMLElement::Rect),
            "stroke" => Ok(VXMLElement::Stroke),
            "fill" => Ok(VXMLElement::Fill),
            "textbox" => Ok(VXMLElement::TextBox),
            _ => Ok(VXMLElement::Unsupported),
        }
    }
}

pub trait ElementReader {
    fn read<R: Read>(r: &mut EventReader<R>, attrs: &[OwnedAttribute]) -> Result<Self, ReaderError>
    where
        Self: std::marker::Sized;
}
