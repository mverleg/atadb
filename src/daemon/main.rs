extern crate atadb;

use atadb::daemon::auth::user::User;
use atadb::daemon::tables::column::Column;
use atadb::daemon::tables::data_type::DataType;
use atadb::daemon::tables::schema::Schema;
use atadb::daemon::tables::table::Table;
use atadb::daemon::util::name::Name;

/// Database process.
fn main() {
    println!("This is the database process");
    Schema::new(
        Name::valid("Person"),
        vec![Table::new(
            Name::valid("Person"),
            vec![
                Column::new(Name::valid("family_name"), DataType::String(Option::None)),
                Column::new(Name::valid("birthday"), DataType::Datetime),
                Column::new(Name::valid("salary"), DataType::Decimal(None, Some(2))),
            ],
        )],
        //        vec![User::new_rw(Name::valid("mark"))],
    );
}
