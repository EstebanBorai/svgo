//! XML Parser for SVG documents

use std::io::{BufReader, Read};

use anyhow::Result;
use xml::{reader::XmlEvent, ParserConfig};

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
                                name: attr.name.local_name.clone(),
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
                        attributes: Vec::new(),
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