use crate::backend::Writer;
use crate::buffer::Buffer;
use crate::error::*;
use crate::util;
use crate::Error::Other;
use core::str::{from_utf8_unchecked, FromStr};
use std::io;
use std::io::{Read, Write};

pub fn find_size() -> Result<(u8, u8)> {
    let mut stdout = io::stdout();
    let mut buf: Vec<u8> = Vec::new();
    buf.push_save()?;
    buf.push_goto(255, 255)?;
    buf.push_position_request()?;
    buf.push_restore()?;
    stdout.submit(buf)?;
    read_position_code()
}

pub fn get_position() -> Result<(u8, u8)> {
    let mut stdout = io::stdout();
    stdout.write_all(b"\x1b[6n").unwrap();
    stdout.flush().unwrap();
    read_position_code()
}

pub fn read_position_code() -> Result<(u8, u8)> {
    let mut buf = [0u8; 64];
    let mut n = 0usize;

    let (y, x) = loop {
        n += std::io::stdin().read(&mut buf[n..n + 2])?;
        let result = get_csi_parameters(&buf[..n]);
        if let Ok((param, end, _tail)) = result {
            assert_eq!(end, b'R');
            break param;
        }
    };

    Ok((x, y))
}

pub fn get_csi_parameters(s: &[u8]) -> Result<((u8, u8), u8, &[u8])> {
    let n_esc = s.windows(2).position(|ss| ss == b"\x1b[").ok_or(Other)?;

    let mut end = None;
    let mut param_tail = s[n_esc + 2..].splitn(3, |c| {
        if end.is_some() {
            false
        } else if *c == b';' {
            true
        } else if *c < b'0' || *c > b'9' {
            end = Some(*c);
            true
        } else {
            false
        }
    });

    let param = (&mut param_tail).take(2);
    let mut param = param
        // .map(|s| unsafe { from_utf8_unchecked(s) })
        .filter_map(|s| {
            if s.is_empty() {
                Some(0)
            } else {
                u8::from_str(unsafe { from_utf8_unchecked(s) }).ok()
            }
        });
    let param = (param.next().ok_or(Other)?, param.next().ok_or(Other)?);

    //  The tail is probably empty, but it should be present,
    //  to assure, again, that the end was found.
    let tail = param_tail.next().ok_or(Other)?;

    // Make sure that the end was found
    // and that preceeding parts are actual parameters
    let end = end.ok_or(Other)?;

    Ok((param, end, tail))
}
