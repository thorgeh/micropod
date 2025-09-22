#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(micropod::io))]
    IO(#[from] std::io::Error),

    #[error("Could not create memfd\n{0}")]
    #[diagnostic(code(micropod::memfd))]
    Memfd(#[source] nix::Error),

    #[error("Could not clone init process\n{0}")]
    #[diagnostic(code(micropod::clone))]
    Clone(#[source] nix::Error),

    #[error("Could not wait for init process\n{0}")]
    #[diagnostic(code(micropod::wait))]
    Wait(#[source] nix::Error),

    #[error(transparent)]
    #[diagnostic(code(micropod::encode))]
    Encode(#[from] bincode::error::EncodeError),
}
