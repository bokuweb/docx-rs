use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    open!(open_custom_properties, "Properties", "xmlns", "xmlns:vt");
    open!(open_property, "property", "fmtid", "pid", "name");
    closed_with_child!(lpwstr, "vt:lpwstr");
}
