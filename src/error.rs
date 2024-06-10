use std::io;

use crate::{backend, buffer};

#[derive(Debug)]
pub enum Error {
    Backend(backend::Error),
    Buffer(buffer::Error),
    Io(io::Error),
    Nom(nom::error::ErrorKind),
    Other,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl<I> From<nom::error::Error<I>> for Error {
    fn from(e: nom::error::Error<I>) -> Self {
        Error::Nom(e.code)
    }
}

impl From<backend::Error> for Error {
    fn from(e: backend::Error) -> Self {
        Error::Backend(e)
    }
}

impl From<buffer::Error> for Error {
    fn from(e: buffer::Error) -> Self {
        Error::Buffer(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
