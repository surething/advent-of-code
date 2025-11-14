use crate::error::AdventError::Nom;
use nom::Finish;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdventError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Nom Error: {0}")]
    Nom(#[from] nom::error::Error<String>),
    #[error("No parent directory found")]
    NoParentDirectory,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Environment error: {0}")]
    Env(#[from] std::env::VarError),
    #[error("Other: {0}")]
    Other(String),
}

pub trait AdventErrorExt<I: ToString, O> {
    fn map_and_finish(self) -> Result<O>;
}

impl<I: ToString, O> AdventErrorExt<I, O> for nom::IResult<I, O, nom::error::Error<I>> {
    fn map_and_finish(self) -> Result<O> {
        self.map_err(|e| e.map_input(|input| input.to_string()))
            .finish()
            .map_err(Nom)
            .map(|(_, o)| o)
    }
}

pub type Result<T> = std::result::Result<T, AdventError>;
