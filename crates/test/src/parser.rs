use std::io::BufWriter;

use svgolib::svg::Svg;

use crate::fixtures::JAPAN_SVG;

#[test]
fn parses_svg_file() {
    let svg = Svg::read(JAPAN_SVG).unwrap();
    let nodes = svg.nodes();

    assert_eq!(nodes.len(), 30);
}

#[test]
fn read_writes_svg() {
    let input_str = String::from_utf8(JAPAN_SVG.to_vec()).unwrap();

    let svg = Svg::read(JAPAN_SVG).unwrap();
    let mut output_bytes: Vec<u8> = Vec::new();
    let buf = BufWriter::new(&mut output_bytes);
    svg.write(buf).unwrap();

    let output_str = String::from_utf8(output_bytes).unwrap();

    assert_eq!(input_str, output_str);
}
