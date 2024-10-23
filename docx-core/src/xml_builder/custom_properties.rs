use super::XMLBuilder;
use super::XmlEvent;

use std::io::Write;

impl<W: Write> XMLBuilder<W> {
    open!(open_custom_properties, "Properties", "xmlns", "xmlns:vt");
    open!(open_property, "property", "fmtid", "pid", "name");
    closed_with_child!(lpwstr, "vt:lpwstr");
}
