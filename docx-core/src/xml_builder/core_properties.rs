use super::XMLBuilder;
use super::XmlEvent;

impl XMLBuilder {
    // i.e. <cp:properties xmlns:vt="http://schemas.openxmlformats.org/package/2006/relationships">
    open!(
        open_core_properties,
        "cp:coreProperties",
        "xmlns:cp",
        "xmlns:dc",
        "xmlns:dcterms",
        "xmlns:dcmitype",
        "xmlns:xsi"
    );
    closed_with_child!(dcterms_created, "dcterms:created", "xsi:type");
    closed_with_child!(dc_creator, "dc:creator");
    closed_with_child!(dc_description, "dc:description");
    closed_with_child!(dc_language, "dc:language");
    closed_with_child!(cp_last_modified_by, "cp:lastModifiedBy");
    closed_with_child!(dcterms_modified, "dcterms:modified", "xsi:type");
    closed_with_child!(cp_revision, "cp:revision");
    closed_with_child!(dc_subject, "dc:subject");
    closed_with_child!(dc_title, "dc:title");
}
