use std::io::{Stdout, Write, Stdin};

use crate::backend;
use crate::backend::Writer;
use std::io;
use std::os::unix::io::AsRawFd;
use raw_tty::TtyWithGuard;

impl Writer for Output {
    type InnerBuffer = Vec<u8>;

    fn take_buffer(&mut self) -> backend::Result<Self::BufferRep> {
        let v = Vec::with_capacity(64);
        Ok((v, self))
    }

    fn submit_buffer(&mut self, buf: Self::InnerBuffer) -> backend::Result<()> {
        self.write_all(buf.as_slice())?;
        self.flush()?;
        Ok(())
    }
}


pub fn get() -> (Input, Output) {
    // Dropping these guards will restore the original terminal mode
    let mut guard_in = raw_tty::TtyWithGuard::new(io::stdin()).unwrap();
    guard_in.set_raw_mode().unwrap();

    let mut guard_out = raw_tty::TtyWithGuard::new(io::stdout()).unwrap();
    guard_out.set_raw_mode().unwrap();

    (guard_in, guard_out)
}

pub type Input = TtyWithGuard<Stdin>;
pub type Output = TtyWithGuard<Stdout>;