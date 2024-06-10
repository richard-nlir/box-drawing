use std::io;
use std::os::unix::io::AsRawFd;

use box_drawing::backend::Writer;
use box_drawing::{buffer, backend};
use box_drawing::error;
use box_drawing::util::read_position_code;
use buffer::Buffer;
use box_drawing::backend::stdio::get;

fn main() -> error::Result<()> {
    let (input, mut output) = backend::stdio::get();

    println!("Hello, world!\r");


    output.take_buffer()?
        .push_size_request()?
        .submit()?;

    let position = read_position_code()?;

    println!("{:?}\r", position);

    Ok(())
}
