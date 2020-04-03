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
    McAlternateContent,
    McChoice,
    McFallback,
    Unsupported,
}

#[derive(PartialEq, Debug)]
pub enum WpXMLElement {
    WpAnchor,
    WpSimplePos,
    WpPositionH,
    WpPosOffset,
    WpPositionV,
    WpExtent,
    WpEffectExtent,
    WpWrapNone,
    WpDocProperty,
    Unsupported,
}

#[derive(PartialEq, Debug)]
pub enum AXMLElement {
    AGraphic,
    AGraphicData,
    AXfrm,
    AOff,
    AExt,
    APrstGeom,
    ASolidFill,
    ALn,
    Unsupported,
}

#[derive(PartialEq, Debug)]
pub enum WpsXMLElement {
    WpsWsp,
    WpsCNvSpProperty,
    WpsSpProperty,
    WpsStyle,
    WpsTxbx,
    WpsBodyPr,
    Unsupported,
}
#[derive(PartialEq, Debug)]
pub enum VXMLElement {
    VRect,
    VStroke,
    VFill,
    VTexbox,
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
            "Drawing" => Ok(XMLElement::Drawing),
            "TxbxContent" => Ok(XMLElement::TxbxContent),
            "Pict" => Ok(XMLElement::Pict),
            _ => Ok(XMLElement::Unsupported),
        }
    }
}

impl FromStr for McXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mcAlternateContent" => Ok(McXMLElement::McAlternateContent),
            "mcChoice" => Ok(McXMLElement::McChoice),
            "mcFallback" => Ok(McXMLElement::McFallback),
            _ => Ok(McXMLElement::Unsupported),
        }
    }
}

impl FromStr for WpXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "wpAnchor" => Ok(WpXMLElement::WpAnchor),
            "wpSimplePos" => Ok(WpXMLElement::WpSimplePos),
            "wpPositionH" => Ok(WpXMLElement::WpPositionH),
            "wpPosOffset" => Ok(WpXMLElement::WpPosOffset),
            "wpPositionV" => Ok(WpXMLElement::WpPositionV),
            "wpExtent" => Ok(WpXMLElement::WpExtent),
            "wpEffectExtent" => Ok(WpXMLElement::WpEffectExtent),
            "wpWrapNone" => Ok(WpXMLElement::WpWrapNone),
            "wpDocPr" => Ok(WpXMLElement::WpDocProperty),
            _ => Ok(WpXMLElement::Unsupported),
        }
    }
}

impl FromStr for AXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AGraphic" => Ok(AXMLElement::AGraphic),
            "AGraphicData" => Ok(AXMLElement::AGraphicData),
            "AXfrm" => Ok(AXMLElement::AXfrm),
            "AOff" => Ok(AXMLElement::AOff),
            "AExt" => Ok(AXMLElement::AExt),
            "APrstGeom" => Ok(AXMLElement::APrstGeom),
            "ASolidFill" => Ok(AXMLElement::ASolidFill),
            "ALn" => Ok(AXMLElement::ALn),
            _ => Ok(AXMLElement::Unsupported),
        }
    }
}

impl FromStr for WpsXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "WpsWsp" => Ok(WpsXMLElement::WpsWsp),
            "WpsCNvSpPr" => Ok(WpsXMLElement::WpsCNvSpProperty),
            "WpsSpPr" => Ok(WpsXMLElement::WpsSpProperty),
            "WpsStyle" => Ok(WpsXMLElement::WpsStyle),
            "WpsTxbx" => Ok(WpsXMLElement::WpsTxbx),
            "WpsBodyPr" => Ok(WpsXMLElement::WpsBodyPr),
            _ => Ok(WpsXMLElement::Unsupported),
        }
    }
}

impl FromStr for VXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VRect" => Ok(VXMLElement::VRect),
            "VStroke" => Ok(VXMLElement::VStroke),
            "VFill" => Ok(VXMLElement::VFill),
            "VTexbox" => Ok(VXMLElement::VTexbox),
            _ => Ok(VXMLElement::Unsupported),
        }
    }
}

pub trait ElementReader {
    fn read<R: Read>(r: &mut EventReader<R>, attrs: &[OwnedAttribute]) -> Result<Self, ReaderError>
    where
        Self: std::marker::Sized;
}
