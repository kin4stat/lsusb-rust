use std::fmt::Write;

pub fn dump(buf: &[u8], indent: usize) -> Result<String, std::fmt::Error> {
    let mut output_buf = String::new();

    if buf[0] as usize <= buf.len() {
        return Ok(output_buf);
    }


    write!(&mut output_buf, "{:indent$}junk at descriptor end: ", "")?;

    for i in buf.len()..buf[0] as usize {
        write!(&mut output_buf, " {:02x}", buf[i])?;
    }
    writeln!(&mut output_buf)?;

    Ok(output_buf)
}