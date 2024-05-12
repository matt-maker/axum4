use core::str;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Greeting {
    greeting: String,
    visitor: String,
    visits: u16,
}

impl Greeting {
    pub(crate) fn new(greeting: &str, visitor: String, visits: u16) -> Self {
        Greeting {
            greeting: greeting.to_string(),
            visitor,
            visits,
        }
    }
}