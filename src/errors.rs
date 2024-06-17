/// Errors that can occur during communication with the game server.
#[derive(Debug, thiserror::Error)]
pub enum MessageError {
    #[error("Bad command with error code {0}.")]
    BadCommand(i32),

    #[error("Invalid reponse.")]
    InvalidResponse,
}
