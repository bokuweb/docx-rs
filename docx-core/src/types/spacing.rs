use serde::ser::{SerializeStruct, Serializer};
use serde::*;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub enum SpacingType {
    Value(i32),
    Line(u32),
}

impl Serialize for SpacingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SpacingType::Value(ref s) => {
                let mut t = serializer.serialize_struct("SpacingType", 2)?;
                t.serialize_field("type", "value")?;
                t.serialize_field("data", s)?;
                t.end()
            }
            SpacingType::Line(ref s) => {
                let mut t = serializer.serialize_struct("SpacingType", 2)?;
                t.serialize_field("type", "line")?;
                t.serialize_field("data", s)?;
                t.end()
            }
        }
    }
}
