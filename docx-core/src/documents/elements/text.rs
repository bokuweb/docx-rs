use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use std::collections::HashMap;

use crate::documents::BuildXML;
use crate::escape::escape;
use crate::xml_builder::*;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub text: String,
    pub preserve_space: bool,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Text {
        Text {
            text: escape(&text.into()),
            preserve_space: true,
        }
    }

    // VAR, e.g. ${VAR}
    pub fn get_vars(&self) -> Vec<String> {
        let mut vars = Vec::new();
        let mut var = String::new();
        let mut in_var = false;
        let mut start = false;
        for c in self.text.chars() {
            if c == '$' {
                in_var = true;
            } else if c == '{' {
                if in_var {
                    start = true;
                    var.clear();
                }
            } else if c == '}' {
                if start {
                    vars.push(var.clone());
                    start = false;
                    in_var = false;
                }
            } else if start {
                var.push(c);
            }
        }
        vars
    }

    pub fn render(&mut self, dictionary: &HashMap<String, String>) {
        let vars = self.get_vars();
        let vars_replace = vars
            .iter()
            .map(|s| dictionary.get(s).unwrap_or(&String::new()).clone())
            .collect::<Vec<_>>();
        for (var, replace) in vars.iter().zip(vars_replace.iter()) {
            self.text = self.text.replace(&format!("${{{}}}", var), replace);
            println!("{}", self.text);
        }
    }

    pub(crate) fn without_escape(text: impl Into<String>) -> Text {
        Text {
            text: text.into(),
            preserve_space: true,
        }
    }
}

impl BuildXML for Text {
    fn build(&self) -> Vec<u8> {
        XMLBuilder::new().text(&self.text, true).build()
    }
}

impl Serialize for Text {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut t = serializer.serialize_struct("Text", 2)?;
        t.serialize_field("preserveSpace", &self.preserve_space)?;
        t.serialize_field("text", &self.text)?;
        t.end()
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
        let b = Text::new("Hello").build();
        assert_eq!(
            str::from_utf8(&b).unwrap(),
            r#"<w:t xml:space="preserve">Hello</w:t>"#
        );
    }

    #[test]
    fn test_json() {
        let t = Text::new("Hello");
        assert_eq!(
            serde_json::to_string(&t).unwrap(),
            r#"{"preserveSpace":true,"text":"Hello"}"#
        );
    }
}
