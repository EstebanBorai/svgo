//! XML Parser for SVG documents

use std::{collections::HashSet, io::{BufReader, Read}};

use anyhow::Result;
use xml::{name::OwnedName, reader::XmlEvent, ParserConfig};

use super::node::{Attribute, Element, ElementType, Node};

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn read<R: Read>(r: R) -> Result<Vec<Node>> {
        let parser = ParserConfig::new()
            .ignore_comments(false)
            .ignore_end_of_stream(false)
            .ignore_root_level_whitespace(false)
            .ignore_invalid_encoding_declarations(false)
            .create_reader(BufReader::new(r));
        let mut els = Vec::new();

        for ev in parser {
            let ev = ev.map_err(|e| anyhow::anyhow!("Error reading XML: {:?}", e))?;

            match ev {
                XmlEvent::StartDocument {
                    version, encoding, ..
                } => {
                    let node = Node::Declaration {
                        version: version.into(),
                        encoding,
                    };

                    els.push(node);
                }
                XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    let element = Element {
                        r#type: ElementType::Open,
                        name: name.local_name,
                        attributes: attributes
                            .iter()
                            .map(|attr| Attribute {
                                name: fully_qualified_attr_name(&attr.name),
                                value: attr.value.clone(),
                            })
                            .collect(),
                    };

                    els.push(Node::Element(element));
                }
                XmlEvent::EndElement { name, .. } => {
                    let element = Element {
                        r#type: ElementType::Close,
                        name: name.local_name,
                        attributes: HashSet::new(),
                    };

                    els.push(Node::Element(element));
                }
                XmlEvent::Comment(value) => {
                    let node = Node::Comment(value);
                    els.push(node);
                }
                XmlEvent::CData(value) => {
                    let node = Node::CData(value);
                    els.push(node);
                }
                XmlEvent::Whitespace(value) | XmlEvent::Characters(value) => {
                    let node = Node::Characters(value);
                    els.push(node);
                }
                XmlEvent::EndDocument => {
                    // Do nothing
                }
                _ => {
                    tracing::warn!("Ignoring event: {:?}", ev);
                }
            }
        }

        Ok(els)
    }
}

impl From<xml::common::XmlVersion> for super::node::Version {
    fn from(v: xml::common::XmlVersion) -> Self {
        match v {
            xml::common::XmlVersion::Version10 => Self::Version10,
            xml::common::XmlVersion::Version11 => Self::Version11,
        }
    }
}

fn fully_qualified_attr_name(on: &OwnedName) -> String {
    if on.namespace.is_some() {
        if let Some(ref prefix) = on.prefix {
            // xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
            return format!("{}:{}", prefix, on.local_name);
        }
    }

    if let Some(ref prefix) = on.prefix {
        // inkscape:version="1.0"
        return format!("{}:{}", prefix, on.local_name);
    }

    // xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
    on.local_name.to_owned()
}
