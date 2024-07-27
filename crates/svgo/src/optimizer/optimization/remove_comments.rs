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
    use std::io::Read;

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

    #[test]
    fn removes_comments_from_svg_string() {
        let raw = r#"<?xml version="1.0" encoding="utf-8"?>
            <!-- Generator: Adobe Illustrator 15.0.0, SVG Export Plug-In . SVG Version: 6.00 Build 0)  -->
            <!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
            <svg version="1.1" xmlns="http://www.w3.org/2000/svg"
                xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
                width="120px" height="120px" viewBox="0 0 120 120"
                enable-background="new 0 0 120 120" xml:space="preserve"
            >
                <style type="text/css"><![CDATA[
                    svg { fill: red; }
                ]]></style>
                <g>
                    <g>
                        <text>  test  </text>
                    </g>
                </g>
                <g style="color: black" class="unknown-class"></g>
            </svg>
        "#;

        let buf = std::io::Cursor::new(raw);
        let mut svg = Svg::read(buf).unwrap();

        let optimization = RemoveCommentsOptimization;
        optimization.apply(&mut svg).unwrap();

        let mut buf = std::io::Cursor::new(Vec::new());
        svg.write(&mut buf).unwrap();

        let mut out = String::new();
        buf.read_to_string(&mut out).unwrap();

        assert_eq!(out, "");
    }
}
