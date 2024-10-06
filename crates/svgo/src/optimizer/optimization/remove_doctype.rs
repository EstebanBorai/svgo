use crate::svg::{node::Node, Svg};

#[derive(Debug, Default, Hash, Eq, PartialEq)]
pub struct RemoveDoctypeOptimization;

impl RemoveDoctypeOptimization {
    pub fn apply(&self, svg: &mut Svg) -> anyhow::Result<()> {
        svg.0.retain(|node| !matches!(node, Node::Doctype(_)));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_comments_from_nodes_vector() {
        let mut svg = Svg(vec![Node::Doctype(String::default())]);

        let optimization = RemoveDoctypeOptimization;
        optimization.apply(&mut svg).unwrap();

        assert_eq!(svg.0.len(), 0);
    }
}
