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
#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

/// Determines wether an element is an opening or closing tag
#[derive(Clone, Debug)]
pub enum ElementType {
    Open,
    Close,
}

/// Elements/Tags in a SVG document tree
#[derive(Clone, Debug)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub r#type: ElementType,
}

/// Nodes in a SVG document
#[derive(Debug)]
pub enum Node {
    /// XML - declaration is not a tag. It is used for the transmission
    /// of the meta-data of a document.
    Declaration {
        /// Used version XML in this document.
        version: Version,
        /// Used encoding in this document.
        encoding: String,
    },
    /// Tag element in a SVG document
    Element(Element),
    /// Comment in a SVG document
    Comment(String),
}
