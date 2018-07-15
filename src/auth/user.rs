use util::name::Name;

#[derive(Debug)]
pub enum RWPerm {
    None,
    Read,
    ReadWrite,
}

// Users without password cannot login.
#[derive(Debug)]
pub struct User {
    name: Name,
    password: Option<String>,
    // todo
    has_perm_grant: bool,
    has_perm_ddl: bool,
    perm_rw: RWPerm,
}

impl User {
    pub fn new_rw(name: Name) -> Self {
        User {
            name,
            password: None,
            has_perm_grant: false,
            has_perm_ddl: false,
            perm_rw: RWPerm::ReadWrite,
        }
    }
}
