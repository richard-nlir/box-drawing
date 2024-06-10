fn read_position_code2() -> Result<(u8, u8)> {

    use nom::bytes::streaming as bs;
    use nom::character::streaming as cs;
    use nom::sequence as seq;
    use nom::Err as Ne;

    let mut buf = [0u8; 64];
    let mut n = 0usize;

    let n = std::io::stdin().read(&mut buf)?;
    let (a,b) = buf.split_at(n);

    let (a, leader) = match bs::is_not("\x1b")(a) {
        Ok(x) => x,
        Err(Ne::Incomplete(_)) => todo!(),
    };


    // let parser = seq::tuple::<_, _, nom::error::Error<_>, _>((
    //     bs::is_not("\x1b"),
    //     bs::tag(b"\x1b["),
    //     seq::separated_pair(cs::digit1, bs::tag(b";"), cs::digit1),
    //     cs::char('R'),
    // ));
    //
    // let param = loop {
    //     let nn = {
    //         let buf = &mut buf[n..];
    //         std::io::stdin().read(buf)?
    //     };
    //     n += nn;
    //     let res = parser(&buf[..n]);
    //     let param = match res {
    //         Err(nom::Err::Incomplete(_)) => continue,
    //         Ok((rest, (leader, csi, param, name))) => param,
    //         Err(nom::Err::Error(e)) |
    //         Err(nom::Err::Failure(e)) => return Err(e.into()),
    //     };
    //     break param;
    // };

    // let (csi, pt) = buf.split_at(2);
    // let (tail, param) = pt.split_last().unwrap();
    //
    // assert_eq!(csi, b"\x1b[");
    // assert!(tail.is_ascii_alphabetic());
    // assert!(param.is_ascii());
    //
    // let args: Vec<u8> = unsafe { from_utf8_unchecked(param) }
    //     .split(';')
    //     .map(u8::from_str)
    //     .map(core::result::Result::unwrap)
    //     .collect();

    let (y, x) = todo!();
    // let mut map = [x, y].iter().copied()
    //     .map(|s| unsafe { from_utf8_unchecked(s) })
    //     .map(u8::from_str)
    //     .map(core::result::Result::unwrap);
    // let x = map.next().unwrap();
    // let y = map.next().unwrap();


    Ok((x, y))
}
