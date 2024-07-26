pub mod optimize;
pub mod svg;

use std::fs::File;

use anyhow::Result;

use svg::node::Node;
use svg::parser::Parser;
use svg::writer::Writer;

pub struct SvgOptimizer {
    nodes: Vec<Node>,
}

impl SvgOptimizer {
    /// Opens a SVG file from a [`File`] and creates an instance of [`SvgOptimizer`]
    /// with it if valid.
    pub fn open(buf: File) -> Result<Self> {
        let nodes = Parser::read(buf)?;

        Ok(Self { nodes })
    }

    /// Writes the SVG document to a [`std::io::Write`] instance.
    pub fn write<W: std::io::Write>(&self, write: W) -> Result<()> {
        Writer::write(write, &self.nodes)
    }
}
