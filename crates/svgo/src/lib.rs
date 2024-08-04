pub mod optimizer;
pub mod svg;

use std::fs::File;

use anyhow::Result;

use optimizer::{Optimization, Optimizer};
use svg::Svg;

pub struct SvgOptimizer {
    pub optimizer: Optimizer,
    pub svg: Svg,
}

impl SvgOptimizer {
    pub fn new(svg: Svg, optimizer: Optimizer) -> Self {
        Self { optimizer, svg }
    }

    /// Opens a SVG file from a [`File`] and creates an instance of [`SvgOptimizer`]
    /// with it if valid.
    pub fn open(buf: File) -> Result<Self> {
        let svg = Svg::open(buf)?;
        let optimizer = Optimizer::default();

        Ok(Self::new(svg, optimizer))
    }

    pub fn read<R: std::io::Read>(read: R) -> Result<Self> {
        let svg = Svg::read(read)?;
        let optimizer = Optimizer::default();

        Ok(Self::new(svg, optimizer))
    }

    /// Writes the underlying [`Svg`] to a [`std::io::Write`] instance.
    pub fn write<W: std::io::Write>(&self, write: W) -> Result<()> {
        self.svg.write(write)
    }

    /// Performs the optimizations on the SVG.
    pub fn optimize(&mut self) -> Result<()> {
        self.optimizer.apply(&mut self.svg)
    }

    /// Appends an [`Optimization`] to be performed on the SVG when
    /// the [`optimize`] method is called.
    pub fn add_optimization(&mut self, optim: Optimization) {
        self.optimizer.append(optim);
    }
}
