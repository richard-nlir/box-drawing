use crate::backend;

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
    fn new() -> Result<Self>;
    fn push_byte(&mut self, s: u8) -> Result<()>;
    fn push_slice(&mut self, s: &[u8]) -> Result<()>;
    fn send(self, dst: &mut impl backend::Backend) -> Result<()>;

    fn push_int(&mut self, mut val: u32) -> Result<()> {
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

    fn push_int_aligned(&mut self, mut val: u32, digits: usize, mut spacing: bool) -> Result<()> {
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

    fn push_esc_byte(&mut self, s: u8) -> Result<()> {
        self.push_byte(0x1b)?;
        self.push_byte(s)
    }

    fn push_esc_slice(&mut self, s: &[u8]) -> Result<()> {
        self.push_byte(0x1b)?;
        self.push_slice(s)
    }

    fn push_ics(&mut self, param: &[u16], tail: u8) -> Result<()> {
        self.push_esc_byte(b'[')?;
        for (n, p) in param.iter().enumerate() {
            if n > 0 {
                self.push_byte(b';')?;
            }
            self.push_int(*p as _)?;
        }
        self.push_byte(tail)
    }

    fn push_goto(&mut self, x: u16, y: u16) -> Result<()> {
        self.push_ics(&[x, y], b'H')
    }

    fn push_save(&mut self) -> Result<()> {
        self.push_ics(&[], b's')
    }
    fn push_restore(&mut self) -> Result<()> {
        self.push_ics(&[], b'u')
    }
    fn push_erase_visible(&mut self) -> Result<()> {
        self.push_ics(&[2], b'J')
    }
    fn push_position_request(&mut self) -> Result<()> {
        self.push_ics(&[6], b'n')
    }
}

impl Buffer for Vec<u8> {
    fn new() -> Result<Self> {
        Ok(Vec::with_capacity(32))
    }

    fn push_byte(&mut self, s: u8) -> Result<()> {
        self.push(s);
        Ok(())
    }

    fn push_slice(&mut self, s: &[u8]) -> Result<()> {
        self.extend_from_slice(s);
        Ok(())
    }

    fn send(self, dst: &mut impl backend::Backend) -> Result<()> {
        dst.submit(self.as_slice())?;
        Ok(())
    }
}
