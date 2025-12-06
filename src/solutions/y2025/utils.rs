use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub struct ParseError(pub String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}