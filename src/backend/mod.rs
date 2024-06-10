use std::io::{self, Stdout, Write};
use crate::buffer::Buffer;

pub mod stdio;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Other,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Writer<'a> {
    type InnerBuffer: Buffer;
    type BufferRep = (Self::InnerBuffer, &'a mut Self);
    fn take_buffer(&mut self) -> Result<Self::BufferRep>;
    fn submit_buffer(&mut self, buf: Self::InnerBuffer) -> Result<()>;
}
