use std::collections::HashSet;

use crate::svg::node::Node;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Optimization {
    RemoveComments,
    RemoveDoctype,
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

    pub fn apply(self, tokens: Vec<Node>) -> anyhow::Result<Vec<Node>> {
        let tokens = tokens;

        for optimization in &self.optimizations {
            todo!()
        }

        Ok(tokens)
    }
}
