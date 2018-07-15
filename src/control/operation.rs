/// The request type is important for the type of lock.
#[derive(Debug, Serialize, Deserialize)]
pub enum Operation {
    DDL,    // Locks schema and data
    Modify, // Locks data for writing (no readers allowed)
    Read,   // Doesn't lock, but needs no modify locks
}

// TODO @mverleg: should there be something for mixed select/write queries? the select part could be done in parallel before locking down to write

#[derive(Debug, Serialize, Deserialize)]
pub struct DDLOperation {}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModifyOperation {
    Insert,
    Delete,
    Update,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectOperation {}
