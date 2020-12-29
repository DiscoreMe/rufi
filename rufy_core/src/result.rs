use thiserror::Error as IError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(IError, Debug)]
pub enum Error {
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[error("lua error")]
    Lua(#[from] rlua::Error),
}