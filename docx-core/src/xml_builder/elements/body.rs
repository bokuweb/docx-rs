use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <w:body... >
    opened_el!(open_body, "w:body");
}
