extern crate atadb;

use atadb::tables::table::Table;
use atadb::util::name::Name;

fn main() {
    println!("Hello, world!");
    Table::new(Name::new("Person".to_owned()).unwrap(), vec![]);
}
