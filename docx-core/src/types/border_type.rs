//
// Please see p3813 <xsd:simpleType name="ST_Border">
//
use serde::{Deserialize, Serialize};
use std::fmt;
use wasm_bindgen::prelude::*;

use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BorderType {
    Nil,
    None,
    Single,
    Thick,
    Double,
    Dotted,
    Dashed,
    DotDash,
    DotDotDash,
    Triple,
    ThinThickSmallGap,
    ThickThinSmallGap,
    ThinThickThinSmallGap,
    ThinThickMediumGap,
    ThickThinMediumGap,
    ThinThickThinMediumGap,
    ThinThickLargeGap,
    ThickThinLargeGap,
    ThinThickThinLargeGap,
    Wave,
    DoubleWave,
    DashSmallGap,
    DashDotStroked,
    ThreeDEmboss,
    ThreeDEngrave,
    Outset,
    Inset,
    Apples,
    ArchedScallops,
    BabyPacifier,
    BabyRattle,
}

impl fmt::Display for BorderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BorderType::Nil => write!(f, "nil"),
            BorderType::None => write!(f, "none"),
            BorderType::Single => write!(f, "single"),
            BorderType::Thick => write!(f, "thick"),
            BorderType::Double => write!(f, "double"),
            BorderType::Dotted => write!(f, "dotted"),
            BorderType::Dashed => write!(f, "dashed"),
            BorderType::DotDash => write!(f, "dotDash"),
            BorderType::DotDotDash => write!(f, "dotDotDash"),
            BorderType::Triple => write!(f, "triple"),
            BorderType::ThinThickSmallGap => write!(f, "thinThickSmallGap"),
            BorderType::ThickThinSmallGap => write!(f, "thickThinSmallGap"),
            BorderType::ThinThickThinSmallGap => write!(f, "thinThickThinSmallGap"),
            BorderType::ThinThickMediumGap => write!(f, "thinThickMediumGap"),
            BorderType::ThickThinMediumGap => write!(f, "thickThinMediumGap"),
            BorderType::ThinThickThinMediumGap => write!(f, "thinThickThinMediumGap"),
            BorderType::ThinThickLargeGap => write!(f, "thinThickLargeGap"),
            BorderType::ThickThinLargeGap => write!(f, "thickThinLargeGap"),
            BorderType::ThinThickThinLargeGap => write!(f, "thinThickThinLargeGap"),
            BorderType::Wave => write!(f, "wave"),
            BorderType::DoubleWave => write!(f, "doubleWave"),
            BorderType::DashSmallGap => write!(f, "dashSmallGap"),
            BorderType::DashDotStroked => write!(f, "dashDotStroked"),
            BorderType::ThreeDEmboss => write!(f, "threeDEmboss"),
            BorderType::ThreeDEngrave => write!(f, "threeDEngrave"),
            BorderType::Outset => write!(f, "outset"),
            BorderType::Inset => write!(f, "inset"),
            BorderType::Apples => write!(f, "apples"),
            BorderType::ArchedScallops => write!(f, "archedScallops"),
            BorderType::BabyPacifier => write!(f, "babyPacifier"),
            BorderType::BabyRattle => write!(f, "babyRattle"),
        }
    }
}

impl FromStr for BorderType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nil" => Ok(BorderType::Nil),
            "none" => Ok(BorderType::None),
            "single" => Ok(BorderType::Single),
            "thick" => Ok(BorderType::Thick),
            "double" => Ok(BorderType::Double),
            "dotted" => Ok(BorderType::Dotted),
            "dashed" => Ok(BorderType::Dashed),
            "dotDash" => Ok(BorderType::DotDash),
            "dotDotDash" => Ok(BorderType::DotDotDash),
            "triple" => Ok(BorderType::Triple),
            "thinThickSmallGap" => Ok(BorderType::ThinThickSmallGap),
            "thickThinSmallGap" => Ok(BorderType::ThickThinSmallGap),
            "thinThickThinSmallGap" => Ok(BorderType::ThinThickThinSmallGap),
            "thinThickMediumGap" => Ok(BorderType::ThinThickMediumGap),
            "thickThinMediumGap" => Ok(BorderType::ThickThinMediumGap),
            "thinThickThinMediumGap" => Ok(BorderType::ThinThickThinMediumGap),
            "thinThickLargeGap" => Ok(BorderType::ThinThickLargeGap),
            "thickThinLargeGap" => Ok(BorderType::ThickThinLargeGap),
            "thinThickThinLargeGap" => Ok(BorderType::ThinThickThinLargeGap),
            "wave" => Ok(BorderType::Wave),
            "doubleWave" => Ok(BorderType::DoubleWave),
            "dashSmallGap" => Ok(BorderType::DashSmallGap),
            "dashDotStroked" => Ok(BorderType::DashDotStroked),
            "threeDEmboss" => Ok(BorderType::ThreeDEmboss),
            "threeDEngrave" => Ok(BorderType::ThreeDEngrave),
            "outset" => Ok(BorderType::Outset),
            "inset" => Ok(BorderType::Inset),
            "apples" => Ok(BorderType::Apples),
            "archedScallops" => Ok(BorderType::ArchedScallops),
            "babyPacifier" => Ok(BorderType::BabyPacifier),
            "babyRattle" => Ok(BorderType::BabyRattle),
            _ => Ok(BorderType::Single),
        }
    }
}

/*
Unsupported types
"balloons3Colors"
"balloonsHotAir"
"basicBlackDashes"
"basicBlackDots"
"basicBlackSquares"
"basicThinLines"
"basicWhiteDashes"
"basicWhiteDots"
"basicWhiteSquares"
"basicWideInline"
"basicWideMidline"
"basicWideOutline"
"bats"
"birds"
"birdsFlight"
"cabins"
"cakeSlice"
"candyCorn"
"celticKnotwork"
"certificateBanner"
"chainLink"
"champagneBottle"
"checkedBarBlack"
"checkedBarColor"
"checkered"
"christmasTree"
"circlesLines"
"circlesRectangles"
"classicalWave"
"clocks"
"compass"
"confetti"
"confettiGrays"
"confettiOutline"
"confettiStreamers"
"confettiWhite"
"cornerTriangles"
"couponCutoutDashes"
"couponCutoutDots"
"crazyMaze"
"creaturesButterfly"
"creaturesFish"
"creaturesInsects"
"creaturesLadyBug"
"crossStitch"
"cup"
"decoArch"
"decoArchColor"
"decoBlocks"
"diamondsGray"
"doubleD"
"doubleDiamonds"
"earth1"
"earth2"
"earth3"
"eclipsingSquares1"
"eclipsingSquares2"
"eggsBlack"
"fans"
"film"
"firecrackers"
"flowersBlockPrint"
"flowersDaisies"
"flowersModern1"
"flowersModern2"
"flowersPansy"
"flowersRedRose"
"flowersRoses"
"flowersTeacup"
"flowersTiny"
"gems"
"gingerbreadMan"
"gradient"
"handmade1"
"handmade2"
"heartBalloon"
"heartGray"
"hearts"
"heebieJeebies"
"holly"
"houseFunky"
"hypnotic"
"iceCreamCones"
"lightBulb"
"lightning1"
"lightning2"
"mapPins"
"mapleLeaf"
"mapleMuffins"
"marquee"
"marqueeToothed"
"moons"
"mosaic"
"musicNotes"
"northwest"
"ovals"
"packages"
"palmsBlack"
"palmsColor"
"paperClips"
"papyrus"
"partyFavor"
"partyGlass"
"pencils"
"people"
"peopleWaving"
"peopleHats"
"poinsettias"
"postageStamp"
"pumpkin1"
"pushPinNote2"
"pushPinNote1"
"pyramids"
"pyramidsAbove"
"quadrants"
"rings"
"safari"
"sawtooth"
"sawtoothGray"
"scaredCat"
"seattle"
"shadowedSquares"
"sharksTeeth"
"shorebirdTracks"
"skyrocket"
"snowflakeFancy"
"snowflakes"
"sombrero"
"southwest"
"stars"
"starsTop"
"stars3d"
"starsBlack"
"starsShadowed"
"sun"
"swirligig"
"tornPaper"
"tornPaperBlack"
"trees"
"triangleParty"
"triangles"
"triangle1"
"triangle2"
"triangleCircle1"
"triangleCircle2"
"shapes1"
"shapes2"
"twistedLines1"
"twistedLines2"
"vine"
"waveline"
"weavingAngles"
"weavingBraid"
"weavingRibbon"
"weavingStrips"
"whiteFlowers"
"woodwork"
"xIllusions"
"zanyTriangles"
"zigZag"
"zigZagStitch"
"custom"
*/
