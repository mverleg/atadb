use daemon::util::hash::make_salt;

pub struct DBConfig {
    salt: Vec<u8>,
}

impl DBConfig {
    pub fn new() -> Self {
        DBConfig { salt: make_salt() }
    }

    pub fn load() -> Self {
        unimplemented!(); // TODO @mverleg:
    }

    pub fn save(&self) -> Result<(), ()> {
        unimplemented!(); // TODO @mverleg:
    }
}
