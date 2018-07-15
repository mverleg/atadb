use daemon::tables::table::Table;
use daemon::util::name::Name;

#[derive(Debug)]
pub struct Schema {
    name: Name,
    tables: Vec<Table>,
}

impl Schema {
    pub fn new(name: Name, tables: Vec<Table>) -> Self {
        Schema { name, tables }
    }
}
