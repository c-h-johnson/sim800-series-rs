use core::result;

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug, PartialEq)]
pub enum Error {
    IntegerRange,
}
