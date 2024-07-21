use anyhow::Result;
use svg::parser::Event;

pub struct RemoveDoctype;

impl RemoveDoctype {
    pub fn apply(tokens: Vec<Event>) -> Result<Vec<Event>> {
        Ok(tokens
            .into_iter()
            .filter(|t| {
                if let Event::Declaration(decl) = t {
                    return !decl.to_ascii_lowercase().contains("!doctype");
                }

                true
            })
            .collect())
    }
}
