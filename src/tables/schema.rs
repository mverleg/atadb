use tables::table::Table;
use auth::user::User;
use util::name::Name;

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
