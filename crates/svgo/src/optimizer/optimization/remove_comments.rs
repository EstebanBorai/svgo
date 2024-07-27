use crate::svg::{node::Node, Svg};

#[derive(Debug, Default, Hash, Eq, PartialEq)]
pub struct RemoveCommentsOptimization;

impl RemoveCommentsOptimization {
    pub fn apply(&self, svg: &mut Svg) -> anyhow::Result<()> {
        svg.0.retain(|node| !matches!(node, Node::Comment(_)));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_comments_from_nodes_vector() {
        let mut svg = Svg(vec![
            Node::Comment("This is a comment".to_string()),
            Node::Comment("This is another comment".to_string()),
        ]);

        let optimization = RemoveCommentsOptimization;
        optimization.apply(&mut svg).unwrap();

        assert_eq!(svg.0.len(), 0);
    }
}
