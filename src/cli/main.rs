extern crate clap;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use std::env;

/// Command line interface.
pub fn main() {
    // TODO @mverleg: Current options don't allow for creating database on another machine
    // TODO @mverleg: I might do something with stored keys to connect later
    let mut app = App::new(env!("CARGO_PKG_NAME").to_owned() + " command-line interface")
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
        .subcommand(SubCommand::with_name("connect")
            .about("Connect to a database")
            .arg(Arg::with_name("DB")
                .help("The name of the database to connect to")
                .required(true)
                .index(1))
            .arg(Arg::with_name("user")
                .long("user")
                .short("u")
                .takes_value(true)
                .default_value("root")
                .help("The name of a user to connect with"))
            .arg(Arg::with_name("password")
                .long("password")
                .short("p")
                .takes_value(true)
                .help("The password of a user to connect with (careful with shell history; leave this out to be prompted)"))
            .arg(Arg::with_name("timeout")
                .long("timeout")
                .short("t")
                .takes_value(true)
                .default_value("300")
                .help("The inactivity timeout (s) after which you need to reconnect"))
        )
        .subcommand(SubCommand::with_name("disconnect")
            .about("Disconnect from a database")
            .arg(Arg::with_name("DB")
                .help("The name of the database to disconnect from")
                .required(true)
                .index(1))
        )
        // Note: there are no special permissions for create and delete, just OS-level file permissions
        .subcommand(SubCommand::with_name("create")
            .about("Create a new database")
            .arg(Arg::with_name("DB")
                .help("Sets the directory where the database is to be created")
                .required(true)
                .index(1))
            .arg(Arg::with_name("user")
                .long("user")
                .short("u")
                .takes_value(true)
                .default_value("root")
                .help("The name of a user to create"))
            .arg(Arg::with_name("password")
                .long("password")
                .short("p")
                .takes_value(true)
                .help("The password of a user to create (careful with shell history; leave this out to be prompted)"))
            .arg(Arg::with_name("port")
                .long("port")
                .short("P")
                .takes_value(true)
                .default_value("4740")  // TODO @mverleg: increment?
                .help("The port to make the database available at"))
            .arg(Arg::with_name("global")
                .long("global")
                .short("g")
                .help("Make this a shared database (needs root permissions)"))
            .arg(Arg::with_name("path")
                .long("path")
                .takes_value(true)
                .help("The path to store the database at"))
        )
        .subcommand(SubCommand::with_name("delete")
            .about("Delete an existing database (careful!)")
            .arg(Arg::with_name("DB")
                .help("Sets the directory of the database to be deleted")
                .required(true)
                .index(1))
            .arg(Arg::with_name("path")
                .long("path")
                .takes_value(true)
                .help("The path to store the database at"))
            .arg(Arg::with_name("yes")
                .short("y")
                .long("non-interactive")
                .help("Delete without further warning or confirmation"))
        )
        .subcommand(SubCommand::with_name("prepare")
            .about("Prepare an SQL query that can be run with 'query'")
            .arg(Arg::with_name("DB")
                .help("The name of the database to perform the query on")
                .required(true)
                .index(1))
        )
        .subcommand(SubCommand::with_name("query")
            .about("Perform a prepared SQL query on the database")
            .arg(Arg::with_name("DB")
                .help("The name of the database to perform the query on")
                .required(true)
                .index(1))
            .arg(Arg::with_name("PREP")
                .help("The identifier of the prepared statement returned by 'prepare'")
                .required(true)
                .index(2))
            .arg(Arg::with_name("INPUT")
                .help("The values for the prepared statement parameters (required if any)")
                .required(false)
                .index(3))
            .arg(Arg::with_name("input_format")
                .long("input-format")
                .short("i")
                .takes_value(true)
                .default_value("csv")
                // TODO @mverleg: any other format?
                .help("The output format to use: json, csv"))
            .arg(Arg::with_name("output_format")
                .long("output-format")
                .short("o")
                .takes_value(true)
                .default_value("text")
                .help("The output format to use: text, json, xml, csv"))
        )
    ;
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
}
