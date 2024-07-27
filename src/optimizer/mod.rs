pub mod optimization;

use std::collections::HashSet;

use crate::svg::Svg;

use self::optimization::remove_comments::RemoveCommentsOptimization;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Optimization {
    /// Remove all comments from the SVG document.
    RemoveComments(RemoveCommentsOptimization),
}

pub struct Optimizer {
    optimizations: HashSet<Optimization>,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Optimizer {
    pub fn new() -> Self {
        Self {
            optimizations: HashSet::new(),
        }
    }

    pub fn append(&mut self, optimization: Optimization) {
        self.optimizations.insert(optimization);
    }

    pub fn apply(&self, svg: &mut Svg) -> anyhow::Result<()> {
        for optimization in &self.optimizations {
            match optimization {
                Optimization::RemoveComments(optimization) => {
                    optimization.apply(svg)?;
                }
            }
        }

        Ok(())
    }
}
