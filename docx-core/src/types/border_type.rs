//
// Please see p3813 <xsd:simpleType name="ST_Border">
//
use serde::Serialize;
use std::fmt;
use wasm_bindgen::prelude::*;

use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Serialize)]
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
            _ => Ok(BorderType::Single),
        }
    }
}

/*
Unsupported types
"thinThickSmallGap"
"thickThinSmallGap"
"thinThickThinSmallGap"
"thinThickMediumGap"
"thickThinMediumGap"
"thinThickThinMediumGap"
"thinThickLargeGap"
"thickThinLargeGap"
"thinThickThinLargeGap"
"wave"
"doubleWave"
"dashSmallGap"
"dashDotStroked"
"threeDEmboss"
"threeDEngrave"
"outset"
"inset"
"apples"
"archedScallops"
"babyPacifier"
"babyRattle"
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
