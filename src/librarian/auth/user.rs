use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use shared::util::name::Name;

#[derive(Debug)]
pub enum RWPerm {
    None,
    Read,
    ReadWrite,
}

// Use UserRef and ask the user service for info. Advantages:
// - Avoid holding on to duplicate, outdated instances.
// - Able to serialize without including security-sensitive data.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserRef {
    id: Name,
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
    pub fn reference(&self) -> UserRef {
        UserRef {
            id: self.name.clone(),
        }
    }

    /// Constructor is not public because [UserRef] should be used.
    fn new_rw(name: Name) -> UserRef {
        let mut user = User {
            name,
            password: None,
            has_perm_grant: false,
            has_perm_ddl: false,
            perm_rw: RWPerm::ReadWrite,
        };
        // TODO @mverleg: add User somewhere
        user.reference()
    }
}
