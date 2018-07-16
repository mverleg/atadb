extern crate atadb;
extern crate clap;

use atadb::daemon::auth::user::User;
use atadb::daemon::tables::column::Column;
use atadb::daemon::tables::data_type::DataType;
use atadb::daemon::tables::schema::Schema;
use atadb::daemon::tables::table::Table;
use atadb::daemon::util::name::Name;
use clap::Arg;
use clap::SubCommand;
use std::env;
use clap::App;

/// Database process.
fn main() {
    let mut app = App::new(env!("CARGO_PKG_NAME").to_owned() + " Daemon")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("verbosity")
            .long("verbose")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .arg(Arg::with_name("version")
            .long("version")
            .help("Show the version and exit"))
        .subcommand(SubCommand::with_name("create")
            .about("Create a new database")
            .arg(Arg::with_name("INPUT")
                .help("Sets the directory where the database is to be created")
                .required(true)
                .index(1)))
        .subcommand(SubCommand::with_name("delete")
            .about("Delete an existing database (careful!)")
            .arg(Arg::with_name("INPUT")
                .help("Sets the directory of the database to be deleted")
                .required(true)
                .index(1))
            .arg(Arg::with_name("yes")
                .short("y")
                .long("non-interactive")
                .help("Delete without further warning or confirmation")))
        .subcommand(SubCommand::with_name("start")
            .about("Start a server for the database that you can run queries against")
            .arg(Arg::with_name("INPUT")
                .help("Sets the directory of the database is start")
                .required(true)
                .index(1))
        );
    if (env::args().len() <= 1) {
        app.print_help();
        return;
    }
    let matches = app.clone().get_matches();
    if matches.is_present("version") {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    unimplemented!();



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
