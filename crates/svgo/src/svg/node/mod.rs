//! XML Nodes and Elements for SVG documents
//! Read more: https://developer.mozilla.org/en-US/docs/Web/XML/XML_introduction

use std::fmt::{Debug, Display};
use std::str::FromStr;

const XML_VERSION_1_0: &str = "1.0";
const XML_VERSION_1_1: &str = "1.1";

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Version {
    /// XML version 1.0.
    Version10,
    /// XML version 1.1.
    Version11,
}

impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            XML_VERSION_1_0 => Ok(Self::Version10),
            XML_VERSION_1_1 => Ok(Self::Version11),
            _ => Err(anyhow::anyhow!("Invalid XML version")),
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match *self {
            Self::Version10 => XML_VERSION_1_0,
            Self::Version11 => XML_VERSION_1_1,
        };

        write!(f, "{}", v)
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match *self {
            Self::Version10 => XML_VERSION_1_0,
            Self::Version11 => XML_VERSION_1_1,
        };

        write!(f, "{}", v)
    }
}

/// Attributes in a SVG document [Element]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Attribute {
    Local {
        key: String,
        value: String,
    },
    Namespaced {
        key: String,
        value: String,
        namespace: String,
        prefix: Option<String>,
    },
    Declaration {
        key: String,
        value: String,
    },
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local { key, value } | Self::Declaration { key, value } => {
                write!(f, "{}=\"{}\"", key, value)
            }
            Self::Namespaced {
                key, value, prefix, ..
            } => {
                if let Some(p) = prefix {
                    write!(f, "{}:{}=\"{}\"", p, key, value)
                } else {
                    write!(f, "{}=\"{}\"", key, value)
                }
            }
        }
    }
}

impl From<xml::attribute::OwnedAttribute> for Attribute {
    fn from(value: xml::attribute::OwnedAttribute) -> Self {
        if let Some(prefix) = value.name.prefix {
            Self::Namespaced {
                key: value.name.local_name,
                value: value.value,
                prefix: Some(prefix),
                namespace: value.name.namespace.unwrap_or_default(),
            }
        } else {
            Self::Local {
                key: value.name.local_name,
                value: value.value,
            }
        }
    }
}

/// Determines wether an element is an opening or closing tag
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElementType {
    Open,
    Close,
}

/// Elements/Tags in a SVG document tree
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub r#type: ElementType,
}

/// Nodes in a SVG document
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    /// XML - declaration is not a tag. It is used for the transmission
    /// of the meta-data of a document.
    Declaration {
        /// Used version XML in this document.
        version: Version,
        /// Used encoding in this document.
        encoding: String,
    },
    /// The `DOCTYPE` declaration in a SVG document
    Doctype(String),
    /// Tag element in a SVG document
    Element(Element),
    /// `CDATA` section in a SVG document
    CData(String),
    /// Comment in a SVG document
    Comment(String),
    /// Arbitrary characters in a SVG document
    Characters(String),
}

#[cfg(test)]
mod test {
    use super::Attribute;

    #[test]
    fn stringify_local_attributes() {
        let attr = vec![
            (
                Attribute::Local {
                    key: "fill".to_string(),
                    value: "#000000".to_string(),
                },
                "fill=\"#000000\"",
            ),
            (
                Attribute::Local {
                    key: "stroke".to_string(),
                    value: "#000000".to_string(),
                },
                "stroke=\"#000000\"",
            ),
            (
                Attribute::Local {
                    key: "stroke-width".to_string(),
                    value: "1".to_string(),
                },
                "stroke-width=\"1\"",
            ),
            (
                Attribute::Local {
                    key: "fill-opacity".to_string(),
                    value: "1".to_string(),
                },
                "fill-opacity=\"1\"",
            ),
            (
                Attribute::Local {
                    key: "style".to_string(),
                    value: "fill:none;stroke:#000000;stroke-width:13.34506607px;stroke-linecap:round;stroke-linejoin:miter".to_string(),
                },
                "style=\"fill:none;stroke:#000000;stroke-width:13.34506607px;stroke-linecap:round;stroke-linejoin:miter\"",
            ),
        ];
        for (attr, expected) in attr {
            assert_eq!(expected, attr.to_string());
        }
    }
    #[test]
    fn stringify_namespaced_attributes() {
        let attr = vec![
            (
                Attribute::Namespaced {
                    key: "label".to_string(),
                    value: "Layer 1".to_string(),
                    prefix: Some("inkscape".to_string()),
                    namespace: "http://www.inkscape.org/namespaces/inkscape".to_string(),
                },
                "inkscape:label=\"Layer 1\"",
            ),
            (
                Attribute::Namespaced {
                    key: "nodetypes".to_string(),
                    value: "csssscsccscssscccc".to_string(),
                    prefix: Some("sodipodi".to_string()),
                    namespace: "http://inkscape.sourceforge.net/DTD/sodipodi-0.dtd".to_string(),
                },
                "sodipodi:nodetypes=\"csssscsccscssscccc\"",
            ),
        ];
        for (attr, expected) in attr {
            assert_eq!(expected, attr.to_string());
        }
    }
}
