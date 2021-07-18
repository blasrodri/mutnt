use nom::error::ErrorKind;

use crate::errors::QueryError;

/// A Command models the different input that the client can
/// request `MuTNT` to do.
pub struct Command {
    data_structure: String,
    action: CommandAction,
}

pub enum CommandAction {
    Query(Query),
    // Insert(),
    // Remove(),
}

pub enum Query {
    KeyValue { key: String },
    Indexable { index: Index },
}

pub enum Index {
    Numbered(usize),
    First,
    Last,
}

pub struct CommandBuilder;

impl CommandBuilder {
    pub fn new() -> Self {
        CommandBuilder
    }

    pub fn build_perhaps(self) -> Result<Command, QueryError> {
        Err(QueryError::NomError(ErrorKind::Alpha))
    }
}
