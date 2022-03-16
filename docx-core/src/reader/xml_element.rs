use std::io::Read;
use std::str::FromStr;

use xml::attribute::OwnedAttribute;
use xml::reader::EventReader;

use crate::reader::ReaderError;

#[derive(PartialEq, Debug, Clone)]
pub enum XMLElement {
    Body,
    Paragraph,
    ParagraphProperty,
    Run,
    RunProperty,
    Color,
    Underline,
    RunFonts,
    Size,
    SizeCs,
    Spacing,
    Vanish,
    TextBorder,
    Italic,
    ItalicCs,
    Text,
    FieldChar,
    InstrText,
    Highlight,
    VertAlign,
    Bold,
    RunStyle,
    BoldCs,
    Break,
    Tab,
    ParagraphStyle,
    ParagraphPropertyChange,
    RunPropertyChange,
    Indent,
    Name,
    BasedOn,
    Alignment,
    NumberingProperty,
    IndentLevel,
    NumberingId,
    Justification,
    OutlineLvl,
    Insert,
    KeepNext,
    KeepLines,
    PageBreakBefore,
    WidowControl,
    DivId,
    Div,
    DivsChild,
    MarginLeft,
    MarginRight,
    MarginTop,
    MarginBottom,
    Delete,
    DeleteText,
    BookmarkStart,
    BookmarkEnd,
    Comment,
    Comments,
    CommentRangeStart,
    CommentRangeEnd,
    CommentExtended,
    Property,
    CommentsExtended,
    VAlign,
    Shading,
    Strike,
    TextDirection,
    Table,
    TableProperty,
    TableRow,
    TableRowHeight,
    HeightRule,
    TableCell,
    TableCellProperty,
    TableCellWidth,
    TableCellBorders,
    TableVMerge,
    TableGridSpan,
    TableWidth,
    TableIndent,
    TableBorders,
    TableCellMargin,
    TableStyle,
    // Change
    TableGridChange,
    TablePropertyChange,
    TableRowPropertyChange,
    TableCellPropertyChange,
    Top,
    Right,
    End,
    Left,
    Start,
    Bottom,
    InsideH,
    InsideV,
    Tl2br,
    Tr2bl,
    TableGrid,
    GridCol,
    GridAfter,
    WidthAfter,
    GridBefore,
    WidthBefore,
    Style,
    Next,
    Styles,
    Relationship,
    Relationships,
    AbstractNumbering,
    AbstractNumberingId,
    LvlOverride,
    StartOverride,
    Level,
    Numbering,
    Settings,
    Num,
    NumberFormat,
    Suffix,
    LevelText,
    LevelJustification,
    LevelRestart,
    StyleLink,
    NumStyleLink,
    Drawing,
    TxbxContent,
    Pict,
    DocId,
    DocVars,
    DocVar,
    DocGrid,
    DocDefaults,
    DefaultTabStop,
    RunPropertyDefault,
    SectionProperty,
    PageSize,
    PageMargin,
    WebSettings,
    HeaderReference,
    FooterReference,
    TitlePg,
    EvenAndOddHeaders,
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
    FontScheme,
    MajorFont,
    MinorFont,
    Latin,
    Ea,
    Cs,
    Font,
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

pub enum VtXMLElement {
    Lpwstr,
    Unsupported,
}

impl FromStr for XMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "body" => Ok(XMLElement::Body),
            "p" => Ok(XMLElement::Paragraph),
            "pPr" => Ok(XMLElement::ParagraphProperty),
            "r" => Ok(XMLElement::Run),
            "rPr" => Ok(XMLElement::RunProperty),
            "rPrChange" => Ok(XMLElement::RunPropertyChange),
            "color" => Ok(XMLElement::Color),
            "t" => Ok(XMLElement::Text),
            "fldChar" => Ok(XMLElement::FieldChar),
            "instrText" => Ok(XMLElement::InstrText),
            "sz" => Ok(XMLElement::Size),
            "szCs" => Ok(XMLElement::SizeCs),
            "u" => Ok(XMLElement::Underline),
            "pStyle" => Ok(XMLElement::ParagraphStyle),
            "pPrChange" => Ok(XMLElement::ParagraphPropertyChange),
            "highlight" => Ok(XMLElement::Highlight),
            "rStyle" => Ok(XMLElement::RunStyle),
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
            "comments" => Ok(XMLElement::Comments),
            "comment" => Ok(XMLElement::Comment),
            "commentRangeStart" => Ok(XMLElement::CommentRangeStart),
            "commentRangeEnd" => Ok(XMLElement::CommentRangeEnd),
            "commentEx" => Ok(XMLElement::CommentExtended),
            "commentsEx" => Ok(XMLElement::CommentsExtended),
            "shd" => Ok(XMLElement::Shading),
            "property" => Ok(XMLElement::Property),
            "tbl" => Ok(XMLElement::Table),
            "tblPr" => Ok(XMLElement::TableProperty),
            "tr" => Ok(XMLElement::TableRow),
            "trHeight" => Ok(XMLElement::TableRowHeight),
            "hRule" => Ok(XMLElement::HeightRule),
            "tc" => Ok(XMLElement::TableCell),
            "tcPr" => Ok(XMLElement::TableCellProperty),
            "tcW" => Ok(XMLElement::TableCellWidth),
            "tcBorders" => Ok(XMLElement::TableCellBorders),
            "vMerge" => Ok(XMLElement::TableVMerge),
            "gridSpan" => Ok(XMLElement::TableGridSpan),
            "gridAfter" => Ok(XMLElement::GridAfter),
            "wAfter" => Ok(XMLElement::WidthAfter),
            "gridBefore" => Ok(XMLElement::GridBefore),
            "wBefore" => Ok(XMLElement::WidthBefore),
            "textDirection" => Ok(XMLElement::TextDirection),
            "tblW" => Ok(XMLElement::TableWidth),
            "tblInd" => Ok(XMLElement::TableIndent),
            "tblBorders" => Ok(XMLElement::TableBorders),
            "tblCellMar" => Ok(XMLElement::TableCellMargin),
            "tblStyle" => Ok(XMLElement::TableStyle),
            "top" => Ok(XMLElement::Top),
            "right" => Ok(XMLElement::Right),
            "start" => Ok(XMLElement::Start),
            "end" => Ok(XMLElement::End),
            "left" => Ok(XMLElement::Left),
            "bottom" => Ok(XMLElement::Bottom),
            "insideH" => Ok(XMLElement::InsideH),
            "insideV" => Ok(XMLElement::InsideV),
            "tl2br" => Ok(XMLElement::Tl2br),
            "tr2bl" => Ok(XMLElement::Tr2bl),
            "tblGrid" => Ok(XMLElement::TableGrid),
            "tblPrChange" => Ok(XMLElement::TablePropertyChange),
            "trPrChange" => Ok(XMLElement::TableRowPropertyChange),
            "tcPrChange" => Ok(XMLElement::TableCellPropertyChange),
            "tblGridChange" => Ok(XMLElement::TableGridChange),
            "gridCol" => Ok(XMLElement::GridCol),
            "style" => Ok(XMLElement::Style),
            "basedOn" => Ok(XMLElement::BasedOn),
            "bdr" => Ok(XMLElement::TextBorder),
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
            "settings" => Ok(XMLElement::Settings),
            "num" => Ok(XMLElement::Num),
            "numFmt" => Ok(XMLElement::NumberFormat),
            "suff" => Ok(XMLElement::Suffix),
            "lvlText" => Ok(XMLElement::LevelText),
            "lvlRestart" => Ok(XMLElement::LevelRestart),
            "lvlJc" => Ok(XMLElement::LevelJustification),
            "outlineLvl" => Ok(XMLElement::OutlineLvl),
            "numStyleLink" => Ok(XMLElement::NumStyleLink),
            "styleLink" => Ok(XMLElement::StyleLink),
            "vAlign" => Ok(XMLElement::VAlign),
            "drawing" => Ok(XMLElement::Drawing),
            "txbxContent" => Ok(XMLElement::TxbxContent),
            "pict" => Ok(XMLElement::Pict),
            "lvlOverride" => Ok(XMLElement::LvlOverride),
            "startOverride" => Ok(XMLElement::StartOverride),
            "strike" => Ok(XMLElement::Strike),
            "docId" => Ok(XMLElement::DocId),
            "docVar" => Ok(XMLElement::DocVar),
            "docVars" => Ok(XMLElement::DocVars),
            "sectPr" => Ok(XMLElement::SectionProperty),
            "pgSz" => Ok(XMLElement::PageSize),
            "rFonts" => Ok(XMLElement::RunFonts),
            "pgMar" => Ok(XMLElement::PageMargin),
            "docDefaults" => Ok(XMLElement::DocDefaults),
            "docGrid" => Ok(XMLElement::DocGrid),
            "rPrDefault" => Ok(XMLElement::RunPropertyDefault),
            "defaultTabStop" => Ok(XMLElement::DefaultTabStop),
            "divId" => Ok(XMLElement::DivId),
            "div" => Ok(XMLElement::Div),
            "divsChild" => Ok(XMLElement::DivsChild),
            "marLeft" => Ok(XMLElement::MarginLeft),
            "marRight" => Ok(XMLElement::MarginRight),
            "marTop" => Ok(XMLElement::MarginTop),
            "marBottom" => Ok(XMLElement::MarginBottom),
            "webSettings" => Ok(XMLElement::WebSettings),
            "keepNext" => Ok(XMLElement::KeepNext),
            "keepLines" => Ok(XMLElement::KeepLines),
            "pageBreakBefore" => Ok(XMLElement::PageBreakBefore),
            "widowControl" => Ok(XMLElement::WidowControl),
            "headerReference" => Ok(XMLElement::HeaderReference),
            "footerReference" => Ok(XMLElement::FooterReference),
            "titlePg" => Ok(XMLElement::TitlePg),
            "evenAndOddHeaders" => Ok(XMLElement::EvenAndOddHeaders),
            _ => Ok(XMLElement::Unsupported),
        }
    }
}

impl FromStr for McXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AlternateContent" => Ok(McXMLElement::AlternateContent),
            "Choice" => Ok(McXMLElement::Choice),
            "Fallback" => Ok(McXMLElement::Fallback),
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
            "fontScheme" => Ok(AXMLElement::FontScheme),
            "majorFont" => Ok(AXMLElement::MajorFont),
            "minorFont" => Ok(AXMLElement::MinorFont),
            "latin" => Ok(AXMLElement::Latin),
            "ea" => Ok(AXMLElement::Ea),
            "cs" => Ok(AXMLElement::Cs),
            "font" => Ok(AXMLElement::Font),
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

impl FromStr for VtXMLElement {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lpwstr" => Ok(VtXMLElement::Lpwstr),
            _ => Ok(VtXMLElement::Unsupported),
        }
    }
}

pub trait ElementReader {
    fn read<R: Read>(r: &mut EventReader<R>, attrs: &[OwnedAttribute]) -> Result<Self, ReaderError>
    where
        Self: std::marker::Sized;
}
