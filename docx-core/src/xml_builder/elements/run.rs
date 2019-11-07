use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <w:r ... >
    opened_el!(open_run, "w:r");
}
