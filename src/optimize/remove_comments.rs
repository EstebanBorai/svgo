use anyhow::Result;
use svg::parser::Event;

pub struct RemoveComments;

impl RemoveComments {
    pub fn apply(tokens: Vec<Event>) -> Result<Vec<Event>> {
        Ok(tokens
            .into_iter()
            .filter(|t| {
                if let Event::Comment(_) = t {
                    return false;
                }

                true
            })
            .collect())
    }
}
