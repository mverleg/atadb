use util::name::Name;

pub enum RWPerm {
    None,
    Read,
    ReadWrite,
}

// Users without password cannot login.
pub struct User {
    name: Name,
    password: Option<String>,
    // todo
    has_perm_grant: bool,
    has_perm_ddl: bool,
    perm_rw: RWPerm,
}

impl User {
    pub fn new_user(name: Name) -> Self {
        User { name,
            password: None,
            has_perm_grant: false,
            has_perm_ddl: false,
            perm_rw: RWPerm::ReadWrite }
    }
}
