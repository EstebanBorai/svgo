pub mod optimize;

use anyhow::{Context, Result};
use svg::{node::Comment, parser::Event, Document};

pub fn tokens(content: &str) -> Result<Vec<Event>> {
    let parser = svg::read(content).context("Not a valid SVG provided")?;
    Ok(parser.into_iter().collect())
}

pub fn into_document(tokens: Vec<Event>) -> Result<Document> {
    let mut document = Document::new();

    for t in tokens {
        match t {
            Event::Comment(content) => {
                document = document.add(Comment::new(content));
            }
            Event::Tag(name, _, attrs) => {
                if name == "svg" {
                    continue;
                }

                let mut el = svg::node::element::Element::new(name);
                el.get_attributes_mut().extend(attrs);
                document = document.add(el);
            }
            Event::Declaration(content) => {
                document = document.add(svg::node::Text::new(unescape(content)));
            }
            _ => {}
        }
    }

    Ok(document)
}

pub(crate) fn unescape(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}
