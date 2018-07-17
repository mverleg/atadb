use librarian::tables::table::Table;
use shared::util::name::Name;

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
