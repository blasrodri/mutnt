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
