//! Errors

use std::fmt::Debug;

use nom::error::ErrorKind;

#[derive(thiserror::Error, Debug)]
pub enum QueryError {
    #[error("ErrorKind({0:?})")]
    NomError(ErrorKind),
}
