#[cfg(test)]
mod parser;

#[cfg(test)]
mod node;

pub mod fixtures {
    pub const JAPAN_SVG: &[u8] = include_bytes!("../fixtures/japan.svg");
}
