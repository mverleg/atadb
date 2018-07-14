use tables::table::Table;
use auth::user::User;
use util::name::Name;

#[derive(Debug)]
pub struct Database {
    name: Name,
    tables: Vec<Table>,
    users: Vec<User>,
}

impl Database {
    pub fn new(name: Name, tables: Vec<Table>, users: Vec<User>) -> Self {
        Database { name, tables, users }
    }
}
