use std::io::{self, Stdout, Write};

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

pub trait Backend {
    fn submit(&mut self, buf: &[u8]) -> Result<()>;
}

impl Backend for Stdout {
    fn submit(&mut self, buf: &[u8]) -> Result<()> {
        self.write_all(buf)?;
        self.flush()?;
        Ok(())
    }
}
