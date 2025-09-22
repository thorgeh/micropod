#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(micropod::io))]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    #[diagnostic(code(micropod::encode))]
    Encode(#[from] bincode::error::EncodeError),
}
