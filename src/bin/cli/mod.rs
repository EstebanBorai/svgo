use std::{fs::File, path::PathBuf};

use anyhow::Result;
use clap::Parser;

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
            let buf = File::open(&file)?;
            let nodes = svgolib::open(buf)?;
            svgolib::printout(nodes)?;
        }

        Ok(())
    }
}
