use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <w:p ... >
    opened_el!(open_paragraph, "w:p");
}
