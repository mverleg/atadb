use auth::user::User;
use control::dbconfig::DBConfig;
use std::marker::PhantomData;
use std::path::PathBuf;
use tables::schema::Schema;

pub struct Database {
    dir: PathBuf,
    config: DBConfig,
    schema: Schema,
    users: Vec<User>,
    data: PhantomData<u8>, // todo
}

impl Database {
    //    pub fn load(name: String) -> Self {
    //        make_dirs();
    //    }
    //
    pub fn load_from(path: PathBuf) -> Self {
        //        make_dirs();
        if !path.exists() {
            panic!("Tried to open non-existent database");
        }
        unimplemented!("TODO"); // TODO @mverleg:
                                //        Database { dir: path, schema: Schema {} }
    }
    //
    //    pub fn create(name: String) -> Result<Self, Err> {
    //        make_dirs();
    //    }
    //
    //    fn make_dirs() {
    //
    //    }
}
