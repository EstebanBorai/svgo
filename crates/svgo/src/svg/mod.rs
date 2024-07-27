pub mod node;
pub mod parser;
pub mod writer;

use std::fs::File;

use anyhow::Result;

use self::node::Node;
use self::parser::Parser;
use self::writer::Writer;

pub struct Svg(pub(crate) Vec<Node>);

impl Svg {
    /// Opens a SVG file from a [`File`] and creates an instance of [`Svg`]
    /// with it if valid.
    pub fn open(buf: File) -> Result<Self> {
        let nodes = Parser::read(buf)?;

        Ok(Self(nodes))
    }

    /// Writes the SVG document to a [`std::io::Write`] instance.
    pub fn write<W: std::io::Write>(&self, write: W) -> Result<()> {
        Writer::write(write, &self.0)
    }

    /// Returns the nodes in the SVG document
    #[inline]
    pub fn nodes(&self) -> &Vec<Node> {
        &self.0
    }
}