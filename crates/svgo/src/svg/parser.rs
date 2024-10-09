//! XML Parser for SVG documents

use std::io::{BufReader, Read};

use anyhow::Result;
use xml::{namespace::Namespace, reader::XmlEvent, ParserConfig};

use super::node::{Attribute, Element, ElementType, Node};

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn read<R: Read>(r: R) -> Result<Vec<Node>> {
        let mut parser = ParserConfig::new()
            .ignore_comments(false)
            .ignore_end_of_stream(false)
            .ignore_root_level_whitespace(false)
            .ignore_invalid_encoding_declarations(false)
            .create_reader(BufReader::new(r));
        let mut els = Vec::new();
        let mut is_doctype_grabbed = false;

        while let Ok(ev) = parser.next() {
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
                    name,
                    attributes,
                    namespace,
                } => {
                    let mut attributes: Vec<Attribute> = attributes
                        .iter()
                        .map(|attr| Attribute::from(attr.to_owned()))
                        .collect();

                    if name.local_name == "svg" {
                        let Namespace(btree) = namespace;
                        btree.iter().for_each(|(k, v)| {
                            if k == "xmlns" {
                                // Introduced by the XML parser (xml-rs)
                                return;
                            }
                            let key = if k.is_empty() {
                                // Interestingly the XML parser (xml-rs) does not provide the
                                // key for the default namespace. It is empty.
                                "xmlns"
                            } else {
                                k
                            };
                            attributes.push(Attribute::Declaration {
                                key: key.to_owned(),
                                value: v.to_owned(),
                            });
                        });
                    }

                    let element = Element {
                        r#type: ElementType::Open,
                        name: name.local_name,
                        attributes,
                    };

                    if let Some(doctype) = parser.doctype() {
                        if !is_doctype_grabbed {
                            els.push(Node::Doctype(doctype.to_string()));
                            is_doctype_grabbed = true;
                        }
                    }

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
                    break;
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
