use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;
use svgolib::optimize::{Optimization, Optimizer};

#[derive(Debug, Parser)]
#[command(
    name = "svgo",
    about = "SVG Optimizer",
    author = "Esteban Borai <estebanborai@gmail.com> (https://github.com/EstebanBorai/svgo)",
    next_line_help = true
)]
pub struct SvgoCli {
    /// Space separated list of SVGs to optimize
    pub files: Vec<PathBuf>,
}

impl SvgoCli {
    pub fn exec(self) -> Result<()> {
        if self.files.is_empty() {
            return Err(anyhow::anyhow!("No files to optimize"));
        }

        for file in self.files {
            let content = std::fs::read_to_string(&file)?;
            let original = svgolib::tokens(&content)?;
            let mut optimizer = Optimizer::new();

            optimizer.append(Optimization::RemoveDoctype);
            optimizer.append(Optimization::RemoveComments);

            let result = optimizer.apply(original)?;
            let document = svgolib::into_document(result)?;
            let mut buf = Vec::new();

            svg::write(&mut buf, &document).context("Failed to write SVG")?;

            let string = String::from_utf8(buf)?;

            println!("{}", string);
        }

        Ok(())
    }
}
