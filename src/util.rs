use core::str::{from_utf8_unchecked, FromStr};

pub fn get_csi_parameters(
    s: &[u8],
) -> Result<((u16,u16), u8, &[u8]), ()> {
    let n_esc = s.windows(2).position(|ss| ss == b"\x1b[").ok_or(())?;

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
        .filter_map(|s|
            if s.is_empty() {
                Some(0)
            } else {
                u16::from_str(unsafe { from_utf8_unchecked(s) }).ok()
            });
    let param = (
        param.next().ok_or(())?,
        param.next().ok_or(())?,
    );

    //  The tail is probably empty, but it should be present,
    //  to assure, again, that the end was found.
    let tail = param_tail.next().ok_or(())?;

    // Make sure that the end was found
    // and that preceeding parts are actual parameters
    let end = end.ok_or(())?;

    Ok((param, end, tail))
}
