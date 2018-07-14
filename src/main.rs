extern crate atadb;

use atadb::tables::table::Table;
use atadb::util::name::Name;
use atadb::tables::column::Column;
use atadb::tables::data_type::DataType;

fn main() {
    println!("Hello, world!");
    Table::new(Name::new("Person".to_owned()).unwrap(), vec![
        Column::new(Name::new("family_name".to_owned()).unwrap(), DataType::String(Option::None)),
    ]);
}
