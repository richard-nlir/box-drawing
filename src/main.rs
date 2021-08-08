#![allow(unused)]

// mod nomwise;
mod backend;
mod buffer;
mod util;
use buffer::Buffer;
use std::io;
use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;

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

fn main() -> Result<()> {
    // Dropping these guards will restore the original terminal mode
    let mut guard_in = raw_tty::TtyModeGuard::new(io::stdin().as_raw_fd()).unwrap();
    let mut guard_out = raw_tty::TtyModeGuard::new(io::stdout().as_raw_fd()).unwrap();
    guard_in.set_raw_mode().unwrap();
    guard_out.set_raw_mode().unwrap();

    println!("Hello, world!\r");
    let position = find_size()?;
    // erase_all();
    // move_to_origin();
    println!("{:?}\r", position);

    Ok(())
}

fn find_size() -> Result<(u16, u16)> {
    let mut stdout = io::stdout();
    let mut buf: Vec<u8> = Vec::new();
    buf.push_save()?;
    buf.push_goto(999, 989)?;
    buf.push_position_request()?;
    buf.push_restore()?;
    buf.send(&mut stdout)?;
    read_position_code()
}

fn get_position() -> Result<(u16, u16)> {
    let mut stdout = io::stdout();
    stdout.write_all(b"\x1b[6n").unwrap();
    stdout.flush().unwrap();
    read_position_code()
}

fn read_position_code() -> Result<(u16, u16)> {
    let mut buf = [0u8; 64];
    let mut n = 0usize;

    let (y,x) = loop {
        n += std::io::stdin().read(&mut buf[n..n+2])?;
        let result = util::get_csi_parameters(&buf[..n]);
        if let Ok((param, end, _tail)) = result {
            assert_eq!(end, b'R');
            break param;
        }
    };

    Ok((x, y))
}

fn erase_all() {
    print!("\x1b[2J")
}

fn move_to_origin() {
    print!("\x1b[1;1H")
}
