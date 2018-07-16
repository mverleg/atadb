extern crate app_dirs;
extern crate rand;
extern crate clap;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate serde;

pub mod shared {}

pub mod daemon {
    pub mod auth;
    pub mod control;
    pub mod ddl;
    pub mod index;
    pub mod intern;
    pub mod modify;
    pub mod rows;
    pub mod select;
    pub mod tables;
    pub mod util;
}

pub mod cli {}
