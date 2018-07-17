use librarian::auth::user::UserRef;
use librarian::control::operation::Operation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    command: Operation,
    user: UserRef,
}
