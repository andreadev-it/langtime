use std::fmt;

#[derive(Debug)]
pub struct NotParsable;

impl fmt::Display for NotParsable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "String not parsable to a valid datetime")
    }
}

impl std::error::Error for NotParsable {}
