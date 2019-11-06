use super::XMLBuilder;

impl XMLBuilder {
    // i.e. <cp:properties xmlns:vt="http://schemas.openxmlformats.org/package/2006/relationships">
    opened_el!(
        open_core_properties,
        "cp:coreProperties",
        "xmlns:cp",
        "xmlns:dc",
        "xmlns:dcterms",
        "xmlns:dcmitype",
        "xmlns:xsi"
    );
    closed_el_with_child!(dcterms_created, "dcterms:created", "xsi:type");
    closed_el_with_child!(dc_creator, "dc:creator");
    closed_el_with_child!(dc_description, "dc:description");
    closed_el_with_child!(dc_language, "dc:language");
    closed_el_with_child!(cp_last_modified_by, "cp:lastModifiedBy");
    closed_el_with_child!(dcterms_modified, "dcterms:modified", "xsi:type");
    closed_el_with_child!(cp_revision, "cp:revision");
    closed_el_with_child!(dc_subject, "dc:subject");
    closed_el_with_child!(dc_title, "dc:title");
}
