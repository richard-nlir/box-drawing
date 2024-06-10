#![allow(unused)]

use style::Style;

use crate::backend;
use crate::backend::Writer;
use crate::backend::stdio::Output;

pub mod style;

#[derive(Debug)]
pub enum Error {
    Backend(backend::Error),
    Other,
}

impl From<backend::Error> for Error {
    fn from(e: backend::Error) -> Self {
        Error::Backend(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

pub trait Buffer: Sized {

    fn submit(self) -> Result<()>;
    fn push_byte(&mut self, s: u8) -> Result<&mut Self>;
    fn push_slice(&mut self, s: &[u8]) -> Result<&mut Self>;

    fn push_int(&mut self, mut val: u32) -> Result<&mut Self> {
        let mut buf = [0u8; 16];
        let mut val_len = 0;
        for (n, p) in buf[..].iter_mut().rev().enumerate() {
            *p = b'0' + (val % 10) as u8;
            val /= 10;
            if val == 0 {
                val_len = n + 1;
                break;
            }
        }
        self.push_slice(&buf[buf.len() - val_len..])
    }

    fn push_int_aligned(&mut self, mut val: u32, digits: usize, mut spacing: bool) -> Result<&mut Self> {
        let mut buf = [0u8; 16];
        let mut i = 10u32.pow(digits as _);
        let oversized = val >= i;
        for p in buf[..digits].iter_mut() {
            i /= 10;
            *p = if oversized {
                b'#'
            } else if val >= i {
                spacing = false;
                b'0' + (val / i) as u8
            } else if spacing {
                b' '
            } else {
                b'0'
            };
            val %= i;
        }
        self.push_slice(&buf[..digits])
    }

    fn push_esc_byte(&mut self, s: u8) -> Result<&mut Self> {
        self.push_byte(0x1b)?
            .push_byte(s)
    }

    fn push_esc_slice(&mut self, s: &[u8]) -> Result<&mut Self> {
        self.push_byte(0x1b)?
            .push_slice(s)
    }

    fn push_csi(&mut self, param: &[u8], tail: u8) -> Result<&mut Self> {
        self.push_esc_byte(b'[')?;
        for (n, p) in param.iter().enumerate() {
            if n > 0 {
                self.push_byte(b';')?;
            }
            self.push_int(*p as _)?;
        }
        self.push_byte(tail)
    }

    fn push_goto(&mut self, x: u8, y: u8) -> Result<&mut Self> {
        self.push_csi(&[x, y], b'H')
    }

    fn push_save(&mut self) -> Result<&mut Self> {
        self.push_csi(&[], b's')
    }

    fn push_restore(&mut self) -> Result<&mut Self> {
        self.push_csi(&[], b'u')
    }

    fn push_erase_visible(&mut self) -> Result<&mut Self> {
        self.push_csi(&[2], b'J')
    }

    fn push_position_request(&mut self) -> Result<&mut Self> {
        self.push_csi(&[6], b'n')
    }

    fn push_size_request(&mut self) -> Result<&mut Self> {
        self.push_save()?
            .push_goto(255, 255)?
            .push_position_request()?
            .push_restore()
    }

    fn style(&mut self) -> style::Style<Self> {
        style::Style::new(self)
    }
}

impl Buffer for (Vec<u8>, &mut Output)
{
    fn submit(self) -> Result<()> {
        let (v, b) = self;
        b.submit_buffer(v);
        Ok(())
    }

    fn push_byte(&mut self, s: u8) -> Result<&mut Self> {
        self.push(s);
        Ok(self)
    }

    fn push_slice(&mut self, s: &[u8]) -> Result<&mut Self> {
        self.extend_from_slice(s);
        Ok(self)
    }
}
