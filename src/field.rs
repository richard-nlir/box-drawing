#![allow(unused)]

use crate::{
    buffer::style::Color,
    buffer::{Buffer, Result},
};

pub trait Bbox {
    fn bbox(&self) -> (u8, u8);
}
pub trait Draw {
    fn draw<Bu: Buffer>(&self, buf: &mut Bu, position: (u8, u8)) -> Result<()>;
}

pub struct Field {
    label: &'static str,
    value: u32,
    focus: bool,
}

impl Field {
    pub fn new(label: &'static str, value: u32) -> Self {
        Self {
            label,
            value,
            focus: false,
        }
    }
}

impl Draw for Field {
    fn draw<Bu: Buffer>(&self, buf: &mut Bu, position: (u8, u8)) -> Result<()> {
        // go to
        let (x, y) = position;
        buf.push_goto(x, y)?;
        // set color, depending on edit status
        let style = buf.style();
        if self.focus {
            style.colorb(Color::Blue).bold()
        } else {
            style
        }
        .push()?;
        // write label
        buf.push_slice(self.label.as_bytes())?;
        // write value
        buf.push_int_aligned(self.value, 8, true)
    }
}

impl Bbox for Field {
    fn bbox(&self) -> (u8, u8) {
        let width = self.label.chars().count() + 2 + 8;
        (width as _, 1)
    }
}

pub struct Vbox<const N: usize> {
    fields: [Field; N],
}

impl<const N: usize> Draw for Vbox<N> {
    fn draw<Bu: Buffer>(&self, buf: &mut Bu, position: (u8, u8)) -> Result<()> {
        let (x, mut y) = position;
        for f in &self.fields {
            f.draw(buf, (x, y))?;
            let (_, h) = f.bbox();
            y += h;
        }
        Ok(())
    }
}

impl<const N: usize> Bbox for Vbox<N> {
    fn bbox(&self) -> (u8, u8) {
        let mut width = 0;
        let mut height = 0;
        for f in &self.fields {
            let (w, h) = f.bbox();
            width = width.max(w);
            height += h;
        }
        (width, height)
    }
}

pub struct UlMargin<C> {
    left: u8,
    top: u8,
    child: C,
}

impl<C: Bbox> UlMargin<C> {
    pub fn center(child: C, width: u8, height: u8) -> Self {
        let (w, h) = child.bbox();
        Self {
            left: (width - w) / 2,
            top: (height - h) / 2,
            child,
        }
    }
    pub fn as_position(&self) -> (u8, u8) {
        (self.left, self.top)
    }
}

impl<C: Draw + Bbox> Draw for UlMargin<C> {
    fn draw<Bu: Buffer>(&self, buf: &mut Bu, position: (u8, u8)) -> Result<()> {
        self.child.draw(buf, self.as_position())
    }
}
