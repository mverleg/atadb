use tables::table::Table;
use auth::user::User;

pub struct Database {
    tables: Vec<Table>,
    users: Vec<User>,
}
