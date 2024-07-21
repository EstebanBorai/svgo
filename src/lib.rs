pub mod optimize;
pub mod svg;

use std::fs::File;

use anyhow::Result;

pub fn open(buf: File) -> Result<()> {
    let parser = svg::parser::Parser::read(buf)?;

    for el in parser {
        println!("{:?}", el);
    }

    Ok(())
}
