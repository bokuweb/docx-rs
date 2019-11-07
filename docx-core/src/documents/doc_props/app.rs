use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug)]
pub struct AppProps {
    config: Option<AppPropsConfig>,
}

#[derive(Debug)]
pub struct AppPropsConfig {
    template: Option<&'static str>,
    total_time: Option<usize>,
    application: Option<&'static str>,
    pages: Option<usize>,
    words: Option<usize>,
    characters: Option<usize>,
    characters_with_spaces: Option<usize>,
    paragraphs: Option<usize>,
}

impl AppProps {
    pub fn new(config: Option<AppPropsConfig>) -> AppProps {
        AppProps { config }
    }
}

impl BuildXML for AppProps {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        let base = b.declaration(Some(true)).open_properties(
            "http://schemas.openxmlformats.org/officeDocument/2006/extended-properties",
            "http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes",
        );

        let convert = |v: usize| format!("{}", v);
        let default = || "".to_owned();

        if let Some(c) = &self.config {
            base.template(c.template.map_or_else(|| "", |v| v))
                .total_time(&c.total_time.map_or_else(default, convert))
                .application(c.application.map_or_else(|| "", |v| v))
                .pages(&c.pages.map_or_else(default, convert))
                .words(&c.words.map_or_else(default, convert))
                .characters(&c.characters.map_or_else(default, convert))
                .characters_with_spaces(&c.characters_with_spaces.map_or_else(default, convert))
                .paragraphs(&c.paragraphs.map_or_else(default, convert))
                .close()
                .build()
        } else {
            base.template("")
                .total_time("")
                .application("")
                .pages("")
                .words("")
                .characters("")
                .characters_with_spaces("")
                .paragraphs("")
                .close()
                .build()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str;

    #[test]
    fn test_default_doc_props_app_build() {
        let c = AppProps::new(None);
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
  <Template></Template>
  <TotalTime></TotalTime>
  <Application></Application>
  <Pages></Pages>
  <Words></Words>
  <Characters></Characters>
  <CharactersWithSpaces></CharactersWithSpaces>
  <Paragraphs></Paragraphs>
</Properties>"#
        );
    }

    #[test]
    fn test_configured_doc_props_app_build() {
        let c = AppProps::new(Some(AppPropsConfig {
            template: Some("temp"),
            total_time: Some(10),
            application: Some("Lawgue beta1.0"),
            pages: Some(1),
            words: Some(20),
            characters: Some(10),
            characters_with_spaces: Some(22),
            paragraphs: Some(30),
        }));
        let b = c.build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties" xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
  <Template>temp</Template>
  <TotalTime>10</TotalTime>
  <Application>Lawgue beta1.0</Application>
  <Pages>1</Pages>
  <Words>20</Words>
  <Characters>10</Characters>
  <CharactersWithSpaces>22</CharactersWithSpaces>
  <Paragraphs>30</Paragraphs>
</Properties>"#
        );
    }
}
