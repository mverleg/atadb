extern crate atadb;

use atadb::tables::data_type::DataType;
use atadb::tables::schema::Schema;
use atadb::tables::table::Table;
use atadb::tables::column::Column;
use atadb::util::name::Name;
use atadb::auth::user::User;

fn main() {
    Schema::new(Name::valid("Person"), vec![
        Table::new(Name::valid("Person"), vec![
            Column::new(Name::valid("family_name"), DataType::String(Option::None)),
            Column::new(Name::valid("birthday"), DataType::Datetime),
            Column::new(Name::valid("salary"), DataType::Decimal(None, Some(2))),
        ])
    ],
                vec![
        User::new_rw(Name::valid("mark")),
    ],);
}
