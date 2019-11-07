use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <w:t ... >
    closed_el_with_child!(text, "w:t");
}
