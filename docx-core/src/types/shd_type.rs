use serde::{Deserialize, Serialize};

use std::fmt;
use std::str::FromStr;
use wasm_bindgen::prelude::*;

use super::errors;

/*
<xsd:enumeration value="nil"/>
<xsd:enumeration value="clear"/>
<xsd:enumeration value="solid"/>
<xsd:enumeration value="horzStripe"/>
<xsd:enumeration value="vertStripe"/>
<xsd:enumeration value="reverseDiagStripe"/>
<xsd:enumeration value="diagStripe"/>
<xsd:enumeration value="horzCross"/>
<xsd:enumeration value="diagCross"/>
<xsd:enumeration value="thinHorzStripe"/>
<xsd:enumeration value="thinVertStripe"/>
<xsd:enumeration value="thinReverseDiagStripe"/>
<xsd:enumeration value="thinDiagStripe"/>
<xsd:enumeration value="thinHorzCross"/>
<xsd:enumeration value="thinDiagCross"/>
<xsd:enumeration value="pct5"/>
<xsd:enumeration value="pct10"/>
<xsd:enumeration value="pct12"/>
<xsd:enumeration value="pct15"/>
<xsd:enumeration value="pct20"/>
<xsd:enumeration value="pct25"/>
<xsd:enumeration value="pct30"/>
<xsd:enumeration value="pct35"/>
<xsd:enumeration value="pct37"/>
<xsd:enumeration value="pct40"/>
<xsd:enumeration value="pct45"/>
<xsd:enumeration value="pct50"/>
<xsd:enumeration value="pct55"/>
<xsd:enumeration value="pct60"/>
<xsd:enumeration value="pct62"/>
<xsd:enumeration value="pct65"/>
<xsd:enumeration value="pct70"/>
<xsd:enumeration value="pct75"/>
<xsd:enumeration value="pct80"/>
<xsd:enumeration value="pct85"/>
<xsd:enumeration value="pct87"/>
<xsd:enumeration value="pct90"/>
<xsd:enumeration value="pct95"/>
*/
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ShdType {
    Nil,
    Clear,
    Solid,
    HorzStripe,
    VertStripe,
    ReverseDiagStripe,
    DiagStripe,
    HorzCross,
    DiagCross,
    ThinHorzStripe,
    ThinVertStripe,
    ThinReverseDiagStripe,
    ThinDiagStripe,
    ThinHorzCross,
    ThinDiagCross,
    Pct5,
    Pct10,
    Pct12,
    Pct15,
    Pct20,
    Pct25,
    Pct30,
    Pct35,
    Pct37,
    Pct40,
    Pct45,
    Pct50,
    Pct55,
    Pct60,
    Pct62,
    Pct65,
    Pct70,
    Pct75,
    Pct80,
    Pct85,
    Pct87,
    Pct90,
    Pct95,
}

impl fmt::Display for ShdType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShdType::Nil => write!(f, "nil"),
            ShdType::Clear => write!(f, "clear"),
            ShdType::Solid => write!(f, "solid"),
            ShdType::HorzStripe => write!(f, "horzStripe"),
            ShdType::VertStripe => write!(f, "vertStripe"),
            ShdType::ReverseDiagStripe => write!(f, "reverseDiagStripe"),
            ShdType::DiagStripe => write!(f, "diagStripe"),
            ShdType::HorzCross => write!(f, "horzCross"),
            ShdType::DiagCross => write!(f, "diagCross"),
            ShdType::ThinHorzStripe => write!(f, "thinHorzStripe"),
            ShdType::ThinVertStripe => write!(f, "thinVertStripe"),
            ShdType::ThinReverseDiagStripe => write!(f, "thinReverseDiagStripe"),
            ShdType::ThinDiagStripe => write!(f, "thinDiagStripe"),
            ShdType::ThinHorzCross => write!(f, "thinHorzCross"),
            ShdType::ThinDiagCross => write!(f, "thinDiagCross"),
            ShdType::Pct5 => write!(f, "pct5"),
            ShdType::Pct10 => write!(f, "pct10"),
            ShdType::Pct12 => write!(f, "pct12"),
            ShdType::Pct15 => write!(f, "pct15"),
            ShdType::Pct20 => write!(f, "pct20"),
            ShdType::Pct25 => write!(f, "pct25"),
            ShdType::Pct30 => write!(f, "pct30"),
            ShdType::Pct35 => write!(f, "pct35"),
            ShdType::Pct37 => write!(f, "pct37"),
            ShdType::Pct40 => write!(f, "pct40"),
            ShdType::Pct45 => write!(f, "pct45"),
            ShdType::Pct50 => write!(f, "pct50"),
            ShdType::Pct55 => write!(f, "pct55"),
            ShdType::Pct60 => write!(f, "pct60"),
            ShdType::Pct62 => write!(f, "pct62"),
            ShdType::Pct65 => write!(f, "pct65"),
            ShdType::Pct70 => write!(f, "pct70"),
            ShdType::Pct75 => write!(f, "pct75"),
            ShdType::Pct80 => write!(f, "pct80"),
            ShdType::Pct85 => write!(f, "pct85"),
            ShdType::Pct87 => write!(f, "pct87"),
            ShdType::Pct90 => write!(f, "pct90"),
            ShdType::Pct95 => write!(f, "pct95"),
        }
    }
}

impl FromStr for ShdType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nil" => Ok(ShdType::Nil),
            "clear" => Ok(ShdType::Clear),
            "solid" => Ok(ShdType::Solid),
            "horzStripe" => Ok(ShdType::HorzStripe),
            "vertStripe" => Ok(ShdType::VertStripe),
            "reverseDiagStripe" => Ok(ShdType::ReverseDiagStripe),
            "diagStripe" => Ok(ShdType::DiagStripe),
            "horzCross" => Ok(ShdType::HorzCross),
            "diagCross" => Ok(ShdType::DiagCross),
            "thinHorzStripe" => Ok(ShdType::ThinHorzStripe),
            "thinVertStripe" => Ok(ShdType::ThinVertStripe),
            "thinReverseDiagStripe" => Ok(ShdType::ThinReverseDiagStripe),
            "thinDiagStripe" => Ok(ShdType::ThinDiagStripe),
            "thinHorzCross" => Ok(ShdType::ThinHorzCross),
            "thinDiagCross" => Ok(ShdType::ThinDiagCross),
            "pct5" => Ok(ShdType::Pct5),
            "pct10" => Ok(ShdType::Pct10),
            "pct12" => Ok(ShdType::Pct12),
            "pct15" => Ok(ShdType::Pct15),
            "pct20" => Ok(ShdType::Pct20),
            "pct25" => Ok(ShdType::Pct25),
            "pct30" => Ok(ShdType::Pct30),
            "pct35" => Ok(ShdType::Pct35),
            "pct37" => Ok(ShdType::Pct37),
            "pct40" => Ok(ShdType::Pct40),
            "pct45" => Ok(ShdType::Pct45),
            "pct50" => Ok(ShdType::Pct50),
            "pct55" => Ok(ShdType::Pct55),
            "pct60" => Ok(ShdType::Pct60),
            "pct62" => Ok(ShdType::Pct62),
            "pct65" => Ok(ShdType::Pct65),
            "pct70" => Ok(ShdType::Pct70),
            "pct75" => Ok(ShdType::Pct75),
            "pct80" => Ok(ShdType::Pct80),
            "pct85" => Ok(ShdType::Pct85),
            "pct87" => Ok(ShdType::Pct87),
            "pct90" => Ok(ShdType::Pct90),
            "pct95" => Ok(ShdType::Pct95),
            _ => Err(errors::TypeError::Unknown),
        }
    }
}
