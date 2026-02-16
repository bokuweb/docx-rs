#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XmlVersion {
    Version10,
}

impl XmlVersion {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            XmlVersion::Version10 => "1.0",
        }
    }
}
