#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Expected `init` request, found `{0}`")]
    Initialisation(String),
    #[error("Server already initialised")]
    AlreadyInitialised,
}
