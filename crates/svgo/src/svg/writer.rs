use std::{borrow::Cow, io::Write};

use anyhow::{Context, Result};
use xml::{common::XmlVersion, name::Name, namespace::Namespace, writer::XmlEvent, EventWriter};

use super::node::{Attribute, ElementType, Node};

pub struct Writer;

impl Writer {
    pub fn write<W: Write>(out: W, nodes: &Vec<Node>) -> Result<()> {
        let mut writer = EventWriter::new(out);

        for node in nodes {
            match node {
                Node::Declaration { version, .. } => {
                    let version = match version {
                        super::node::Version::Version10 => XmlVersion::Version10,
                        super::node::Version::Version11 => XmlVersion::Version11,
                    };

                    writer
                        .write(XmlEvent::StartDocument {
                            version,
                            encoding: None,
                            standalone: None,
                        })
                        .context("Failed to write SVG Start Document")?;
                }
                Node::Doctype(value) => {
                    writer
                        .inner_mut()
                        .write(value.as_bytes())
                        .context("Failed to write SVG characters")?;
                }
                Node::Element(element) => {
                    let name = element.name.as_str();
                    let name = Name::local(name);
                    let namespace = Cow::Owned(Namespace::empty());
                    let attributes = element
                        .attributes
                        .iter()
                        .map(|attr| match attr {
                            Attribute::Local { key, value } => xml::attribute::Attribute {
                                name: Name::local(key.as_str()),
                                value: value.as_str(),
                            },
                            Attribute::Namespaced {
                                key,
                                value,
                                namespace,
                                prefix,
                            } => xml::attribute::Attribute {
                                name: Name::qualified(
                                    key,
                                    namespace,
                                    prefix.as_ref().map(|p| p.as_str()),
                                ),
                                value: value.as_str(),
                            },
                            Attribute::Declaration { key, value } => xml::attribute::Attribute {
                                name: Name::local(key.as_str()),
                                value: value.as_str(),
                            },
                        })
                        .collect();

                    match element.r#type {
                        ElementType::Open => {
                            writer
                                .write(XmlEvent::StartElement {
                                    name,
                                    attributes,
                                    namespace,
                                })
                                .context("Failed to write SVG Start Element")?;
                        }
                        ElementType::Close => {
                            writer
                                .write(XmlEvent::EndElement { name: None })
                                .context("Failed to write SVG End Element")?;
                        }
                    }
                }
                Node::Comment(value) => {
                    writer
                        .write(XmlEvent::Comment(value))
                        .context("Failed to write SVG comment")?;
                }
                Node::CData(value) => {
                    writer
                        .write(XmlEvent::CData(value))
                        .context("Failed to write SVG CDATA")?;
                }
                Node::Characters(value) => {
                    writer
                        .write(XmlEvent::Characters(value))
                        .context("Failed to write SVG characters")?;
                }
            }
        }

        Ok(())
    }
}
