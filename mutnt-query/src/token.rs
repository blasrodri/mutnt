#[derive(Debug, PartialEq)]
pub enum Token {
    // CommandActionToken
    Get,
    Insert,
    Delete,

    // QueryToken
    KeyValue(String),
    Indexable(usize),

    // Datastructure
    Datastructure(String),
}
