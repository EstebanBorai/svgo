pub mod optimize;
pub mod svg;

use std::{fs::File, io::stdout};

use anyhow::Result;

use svg::node::Node;

pub fn open(buf: File) -> Result<Vec<Node>> {
    let parser = svg::parser::Parser::read(buf)?;

    Ok(parser)
}

pub fn printout(nodes: Vec<Node>) -> Result<()> {
    svg::writer::Writer::write(stdout(), nodes)
}
