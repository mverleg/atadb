use auth::user::User;
use tables::schema::Schema;
use std::marker::PhantomData;
use std::path::Path;

pub struct Database {
    dir: Path,
    schema: Schema,
    users: Vec<User>,
    data: PhantomData<u8>,  // todo
}

impl Database {
    pub fn load(name: String) -> Self {
        make_dirs();
    }

    pub fn load_from(path: Path) -> Self {
        make_dirs();
    }

    pub fn create(name: String) -> Result<Self, Err> {
        make_dirs();
    }

    fn make_dirs() {

    }
}
