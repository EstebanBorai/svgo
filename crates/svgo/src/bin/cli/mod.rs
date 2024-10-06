use std::{fs::File, io::stdout, path::PathBuf};

use anyhow::Result;
use clap::Parser;

use svgo::optimizer::optimization::RemoveCommentsOptimization;
use svgo::optimizer::optimization::RemoveDoctypeOptimization;
use svgo::optimizer::Optimization;

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
    /// Removes Comments from SVG
    #[clap(long)]
    pub remove_comments: bool,
    /// Removes DOCTYPE from SVG
    #[clap(long)]
    pub remove_doctype: bool,
}

impl SvgoCli {
    pub fn exec(self) -> Result<()> {
        if self.files.is_empty() {
            return Err(anyhow::anyhow!("No files to optimize"));
        }

        for file in self.files {
            let buf = File::open(&file)?;
            let mut svgo = svgo::SvgOptimizer::open(buf)?;

            if self.remove_comments {
                svgo.add_optimization(Optimization::RemoveComments(RemoveCommentsOptimization));
            }

            if self.remove_doctype {
                svgo.add_optimization(Optimization::RemoveDoctype(RemoveDoctypeOptimization));
            }

            svgo.optimize()?;
            svgo.write(stdout())?;
        }

        Ok(())
    }
}
