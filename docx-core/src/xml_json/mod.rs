// Licensed under either of
//
// Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
// MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
// at your option.
//
// Contribution
// Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
// use serde::Serialize;
use std::fmt;
use std::io::prelude::*;
use std::io::Cursor;
use std::str::FromStr;
use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;
use xml::namespace::{self, Namespace};
use xml::reader::{EventReader, XmlEvent};

/// An XML Document
#[derive(Debug, Clone)]
pub struct XmlDocument {
    /// Data contained within the parsed XML Document
    pub data: Vec<XmlData>,
}

// Print as JSON
impl fmt::Display for XmlDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        for d in self.data.iter() {
            s = format!("{}{}", s, d);
        }

        s.fmt(f)
    }
}

/// An XML Tag
///
/// For exammple:
///
/// ```XML
/// <foo bar="baz">
///     test text
///     <sub></sub>
/// </foo>
/// ```
#[derive(Debug, Clone)]
pub struct XmlData {
    /// Name of the tag (i.e. "foo")
    pub name: String,
    /// Key-value pairs of the attributes (i.e. ("bar", "baz"))
    pub attributes: Vec<(String, String)>,
    /// Data (i.e. "test text")
    pub data: Option<String>,
    /// Sub elements (i.e. an XML element of "sub")
    pub sub_elements: Vec<XmlData>,
}

// Generate indentation
fn indent(size: usize) -> String {
    const INDENT: &str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

// Get the attributes as a string
fn attributes_to_string(attributes: &[(String, String)]) -> String {
    attributes
        .iter()
        .fold(String::new(), |acc, &(ref k, ref v)| {
            format!("{} {}=\"{}\"", acc, k, v)
        })
}

// Format the XML data as a string
fn format(data: &XmlData, depth: usize) -> String {
    let sub = if data.sub_elements.is_empty() {
        String::new()
    } else {
        let mut sub = "\n".to_string();
        for elmt in data.sub_elements.iter() {
            sub = format!("{}{}", sub, format(elmt, depth + 1));
        }
        sub
    };

    let indt = indent(depth);

    let fmt_data = if let Some(ref d) = data.data {
        format!("\n{}{}", indent(depth + 1), d)
    } else {
        "".to_string()
    };

    format!(
        "{}<{}{}>{}{}\n{}</{}>\n",
        indt,
        data.name,
        attributes_to_string(&data.attributes),
        fmt_data,
        sub,
        indt,
        data.name
    )
}

impl fmt::Display for XmlData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format(self, 0))
    }
}

// Get the XML atributes as a string
fn map_owned_attributes(attrs: Vec<xml::attribute::OwnedAttribute>) -> Vec<(String, String)> {
    attrs
        .into_iter()
        .map(|attr| {
            let fmt_name = if attr.name.prefix.is_some() {
                format!("{}:{}", attr.name.prefix.unwrap(), attr.name.local_name)
            } else {
                attr.name.local_name.clone()
            };
            (fmt_name, attr.value)
        })
        .collect()
}

// Parse the data
fn parse(
    mut data: Vec<XmlEvent>,
    current: Option<XmlData>,
    mut current_vec: Vec<XmlData>,
    trim: bool,
    current_namespace: Namespace,
) -> Result<(Vec<XmlData>, Vec<XmlEvent>), String> {
    if let Some(elmt) = data.pop() {
        match elmt {
            XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            } => {
                let fmt_name = if name.prefix.is_some() {
                    format!("{}:{}", name.prefix.unwrap(), name.local_name)
                } else {
                    name.local_name
                };

                let attributes = if namespace == current_namespace {
                    attributes
                } else {
                    let mut attributes = attributes;
                    let n = namespace.clone();
                    let ns = n
                        .into_iter()
                        .filter(|&(k, v)| {
                            (k != namespace::NS_NO_PREFIX)
                                && (v != namespace::NS_EMPTY_URI)
                                && (v != namespace::NS_XMLNS_URI)
                                && (v != namespace::NS_XML_URI)
                        })
                        .map(|(k, v)| OwnedAttribute {
                            name: OwnedName {
                                local_name: k.to_string(),
                                namespace: Some(v.to_string()),
                                prefix: Some("xmlns".to_string()),
                            },
                            value: v.to_string(),
                        });
                    attributes.extend(ns);
                    attributes
                };

                let inner = XmlData {
                    name: fmt_name,
                    attributes: map_owned_attributes(attributes),
                    data: None,
                    sub_elements: Vec::new(),
                };

                let (inner, rest) = parse(data, Some(inner), Vec::new(), trim, namespace.clone())?;

                if let Some(mut crnt) = current {
                    crnt.sub_elements.extend(inner);
                    parse(rest, Some(crnt), current_vec, trim, namespace)
                } else {
                    current_vec.extend(inner);
                    parse(rest, None, current_vec, trim, namespace)
                }
            }
            XmlEvent::Characters(chr) => {
                let chr = if trim { chr.trim().to_string() } else { chr };
                if let Some(mut crnt) = current {
                    crnt.data = Some(chr);
                    parse(data, Some(crnt), current_vec, trim, current_namespace)
                } else {
                    Err("Invalid form of XML doc".to_string())
                }
            }
            XmlEvent::EndElement { name } => {
                let fmt_name = if name.prefix.is_some() {
                    format!("{}:{}", name.prefix.unwrap(), name.local_name)
                } else {
                    name.local_name.clone()
                };
                if let Some(crnt) = current {
                    if crnt.name == fmt_name {
                        current_vec.push(crnt);
                        return Ok((current_vec, data));
                    } else {
                        Err(format!(
                            "Invalid end tag: expected {}, got {}",
                            crnt.name, name.local_name
                        ))
                    }
                } else {
                    Err(format!("Invalid end tag: {}", name.local_name))
                }
            }
            _ => parse(data, current, current_vec, trim, current_namespace),
        }
    } else if let Some(_current) = current {
        Err("Invalid end tag".to_string())
    } else {
        Ok((current_vec, Vec::new()))
    }
}

impl XmlDocument {
    pub fn from_reader<R>(source: R, trim: bool) -> Result<Self, ParseXmlError>
    where
        R: Read,
    {
        let parser = EventReader::new(source);
        let mut events: Vec<XmlEvent> = parser.into_iter().map(|x| x.unwrap()).collect();
        events.reverse();

        parse(events, None, Vec::new(), trim, Namespace::empty())
            .map(|(data, _)| XmlDocument { data })
            .map_err(|e| ParseXmlError(e))
    }
}

/// Error when parsing XML
#[derive(Debug, Clone, PartialEq)]
pub struct ParseXmlError(String);

impl fmt::Display for ParseXmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coult not parse string to XML: {}", self.0)
    }
}

// Generate an XML document from a string
impl FromStr for XmlDocument {
    type Err = ParseXmlError;

    fn from_str(s: &str) -> Result<XmlDocument, ParseXmlError> {
        XmlDocument::from_reader(Cursor::new(s.to_string().into_bytes()), true)
    }
}
