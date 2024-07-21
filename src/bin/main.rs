mod cli;

use anyhow::Result;
use clap::Parser;

use self::cli::SvgoCli;

fn main() -> Result<()> {
    let args = SvgoCli::parse();

    args.exec()?;
    Ok(())
}
