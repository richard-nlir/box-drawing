use box_drawing::backend::Writer;
use box_drawing::buffer::Buffer;
use box_drawing::error::Result;
use box_drawing::field::Draw;
use box_drawing::field::Field;
use box_drawing::field::UlMargin;
use box_drawing::util::read_position_code;
use std::io::stdout;

fn main() -> Result<()> {
    let field1 = Field::new("Position", 4002);
    let mut buf: Vec<u8> = Vec::with_capacity(1 << 10);
    buf.push_size_request()?;
    stdout().submit(buf)?;

    let (width, height) = read_position_code()?;
    let widget = UlMargin::center(Field::new("Position", 223), width, height);
    // widget.draw(&mut buf, (1,1));
    // buf.send(&mut )
    Ok(())
}
