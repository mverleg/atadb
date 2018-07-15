use util::name::Name;
use tables::column::Column;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    name: Name,
    columns: Vec<Column>,
    // TODO @mverleg: constraint
}

impl Table {
    pub fn new(name: Name, columns: Vec<Column>) -> Self {
        Table { name, columns }
    }
}
