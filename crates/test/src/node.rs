use svgo::svg::node::{Attribute, Node};
use svgo::svg::parser::Parser;

use crate::fixtures::JAPAN_SVG;

#[test]
fn parses_svg_local_attributes() {
    let nodes = Parser::read(JAPAN_SVG).expect("Failed to parse SVG");
    let attributes: Vec<Attribute> = nodes
        .into_iter()
        .filter_map(|node| match node {
            Node::Element(element) => Some(element.attributes),
            _ => None,
        })
        .flatten()
        .collect();
    assert_eq!(attributes.len(), 15);
}
