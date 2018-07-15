use auth::user::UserRef;
use control::operation::Operation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    command: Operation,
    user: UserRef,
}
