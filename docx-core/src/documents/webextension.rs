use serde::Serialize;

use crate::documents::BuildXML;
use crate::escape::escape;
use crate::xml_builder::*;

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct WebExtensionProperty {
    pub name: String,
    pub value: String,
}

impl WebExtensionProperty {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct WebExtension {
    pub id: String,
    pub version: String,
    pub store: String,
    pub store_type: String,
    pub reference_id: String,
    pub properties: Vec<WebExtensionProperty>,
}

impl WebExtension {
    pub fn new(
        id: impl Into<String>,
        reference_id: impl Into<String>,
        version: impl Into<String>,
        store: impl Into<String>,
        store_type: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            reference_id: reference_id.into(),
            version: version.into(),
            store: store.into(),
            store_type: store_type.into(),
            properties: vec![],
        }
    }

    pub fn property(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let v = value.into();
        let v = format!("&quot;{}&quot;", escape(&v).replace("&quot;", "\\&quot;"));
        self.properties.push(WebExtensionProperty::new(name, &v));
        self
    }
}

impl BuildXML for WebExtension {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let mut b = b
            .declaration(Some(true))
            .open_webextension(
                "http://schemas.microsoft.com/office/webextensions/webextension/2010/11",
                &format!("{{{}}}", &self.id),
            )
            .webextension_reference(
                &self.reference_id,
                &self.version,
                &self.store,
                &self.store_type,
            )
            .webextension_alternate_references()
            .open_webextension_properties();

        for p in self.properties.iter() {
            b = b.webextension_property(&p.name, &p.value);
        }

        b.close()
            .webextension_bindings()
            .webextension_snapshot(
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
            )
            .close()
            .build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_build() {
        let c = WebExtension::new(
            "7f33b723-fb58-4524-8733-dbedc4b7c095",
            "abcd",
            "1.0.0.0",
            "developer",
            "Registry",
        )
        .property("hello", "world");
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<we:webextension xmlns:we="http://schemas.microsoft.com/office/webextensions/webextension/2010/11" id="{7f33b723-fb58-4524-8733-dbedc4b7c095}">
  <we:reference id="abcd" version="1.0.0.0" store="developer" storeType="Registry" />
  <we:alternateReferences />
  <we:properties>
    <we:property name="hello" value="&quot;world&quot;" />
  </we:properties>
  <we:bindings />
  <we:snapshot xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" />
</we:webextension>"#
        );
    }
}
