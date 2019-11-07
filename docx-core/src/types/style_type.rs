use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum StyleType {
    Paragraph,
    Character,
}

impl fmt::Display for StyleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StyleType::Paragraph => write!(f, "paragraph"),
            StyleType::Character => write!(f, "character"),
        }
    }
}
