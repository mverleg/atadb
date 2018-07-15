use daemon::auth::user::UserRef;
use daemon::control::operation::Operation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    command: Operation,
    user: UserRef,
}
