use std::fmt;
use std::fmt::Formatter;

type Result<T> = std::result::Result<T, NotFoundInList>;

#[derive(Debug, Clone)]
pub struct NotFoundInList;

impl fmt::Display for NotFoundInList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Could not find that task in the list.")
    }
}