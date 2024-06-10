use crate::buffer;
use crate::buffer::Buffer;
use crate::buffer::Result;


pub struct Style<'b, B> {
    buf: &'b mut B,
    arr: [u8; 8],
    inx: usize,
}

impl<'b, B: Buffer> Style<'b, B> {
    pub(crate) fn new(b: &'b mut B) -> Self {
        Self {
            buf: b,
            arr: Default::default(),
            inx: 1,
        }
    }
    pub fn push(self) -> Result<&mut B> {
        let Self { buf, arr, inx } = self;
        buf.push_csi(&arr[..inx], b'm')
    }
    fn push_val(mut self, val: u8) -> Self {
        let Self { arr, inx, .. } = &mut self;
        arr[*inx] = val;
        *inx += 1;
        self
    }
    pub fn reset(self) -> Self {
        self.push_val(0)
    }
    pub fn bold(self) -> Self {
        self.push_val(1)
    }
    pub fn underline(self) -> Self {
        self.push_val(4)
    }
    pub fn blink(self) -> Self {
        self.push_val(5)
    }
    pub fn reverse(self) -> Self {
        self.push_val(7)
    }
    pub fn colorf(self, c: Color) -> Self {
        self.push_val(30 + c as u8)
    }
    pub fn colorfbright(self, c: Color) -> Self {
        self.push_val(90 + c as u8)
    }
    pub fn colorb(self, c: Color) -> Self {
        self.push_val(40 + c as u8)
    }
    pub fn colorbbright(self, c: Color) -> Self {
        self.push_val(100 + c as u8)
    }
}

#[repr(u8)]
pub enum Color {
    Black,
    Red,
    Green,
    Brown,
    Blue,
    Magenta,
    Cyan,
    White,
    Shoehorned,
    Default,
}
