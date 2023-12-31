#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Did not receive `init` request")]
    Initialisation,
    #[error("Server already initialised")]
    AlreadyInitialised,
    #[error("Server should not receive a `{0}` request")]
    InvalidRequest(String),
}
