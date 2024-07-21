use std::collections::HashSet;

mod remove_comments;
mod remove_doctype;

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

    pub fn apply(self, tokens: Vec<svg::parser::Event>) -> anyhow::Result<Vec<svg::parser::Event>> {
        let mut tokens = tokens;

        for optimization in &self.optimizations {
            match optimization {
                Optimization::RemoveComments => {
                    tokens = remove_comments::RemoveComments::apply(tokens)?;
                }
                Optimization::RemoveDoctype => {
                    tokens = remove_doctype::RemoveDoctype::apply(tokens)?;
                }
            }
        }

        Ok(tokens)
    }
}
